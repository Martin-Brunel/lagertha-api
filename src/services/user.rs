use chrono::Utc;
use rocket::http::Status;
use uuid::Uuid;

use crate::{
    db::connect::DbPool, dto::user::{
        user_input::UserInput, user_insertable::UserInsertable, user_public_input::UserPublicInput,
        user_totp_code::UserTotpCode,
    }, models::user::User, repositories::{
        application::ApplicationRepository, user::UserRepository,
    }, traits::{application::ApplicationContract, connexion::ConnexionContract}, utils::{code, crypto::Crypto, password::PasswordUtils, pq_kyber::PQKyber}, LICENSE_VALID
};

use super::mail::MailService;

pub struct UserService<T, D> {
    pool: DbPool,
    user_repository: UserRepository,
    application_repository: T,
    connexion_repository: D,
}

impl<T: ApplicationContract, D: ConnexionContract> UserService<T, D> {
    fn check_if_unique_login(&self, login: &String, app_id: &i32) -> bool {
        let user = self
            .user_repository
            .get_by_login_and_app(login, app_id.clone());
        match user {
            None => true,
            Some(_) => false,
        }
    }

    pub fn new(pool: &DbPool, application_repository: T, connexion_repository: D) -> Self {
        Self {
            pool: pool.clone(),
            user_repository: UserRepository::new(&pool),
            application_repository,
            connexion_repository,
        }
    }

    pub async fn reinit_kyber_keypair(&self, user: &User) -> Result<User, Status> {
        let (public, secret) = PQKyber::generate_key_pair();
        let iv = Crypto::generate_unique_iv();
        let sk = PQKyber::encrypt_key(hex::encode(secret), iv.clone());
        let pk = PQKyber::encrypt_key(hex::encode(public), iv.clone());

        match self.user_repository.update_pq(user.id, sk, pk, iv) {
            Err(_) => Err(Status::BadRequest),
            Ok(_) => {
                let user = self
                    .user_repository
                    .get_by_login_and_app(&user.login, user.application.unwrap())
                    .unwrap();
                Ok(user)
            }
        }
    }

    fn allow_creation(&self) -> Result<(), Status> {
        let license_valid = LICENSE_VALID.lock().unwrap();
        match license_valid.clone() {
            Some(license) => match license.mode == "Entreprise" {
                true => Ok(()),
                false => {
                    let count = self.count_users();
                    if count >= 10_000 {
                        return Err(Status::UpgradeRequired);
                    }
                    Ok(())
                }
            },
            None => {
                let count = self.count_users();
                if count >= 200 {
                    return Err(Status::UpgradeRequired);
                }
                Ok(())
            }
        }
    }

    fn count_users(&self) -> i64 {
        self.user_repository.count_users().unwrap()
    }

    pub async fn create_user_public(&self, user_input: UserPublicInput) -> Result<User, Status> {
        // verification si la license permet la creation de l'utilisateur
        match self.allow_creation() {
            Err(status) => Err(status),
            Ok(_) => {
                let user_repository = UserRepository::new(&self.pool);
                match self
                    .application_repository
                    .get_by_id(user_input.application_id)
                {
                    None => Err(Status::NotFound),
                    Some(app) => {
                        if !self.check_if_unique_login(&user_input.login, &app.id) {
                            return Err(Status::Conflict);
                        };
                        let insertable = UserInsertable::new_public(user_input, &app).await;
                        let user = user_repository.create_user(insertable);
                        let _ = self.application_repository.increment_users(&app.id);
                        Ok(user)
                    }
                }
            }
        }
    }

    pub async fn validate_user(
        &self,
        user: &User,
        validation_code: &String,
    ) -> Result<User, Status> {
        match user.is_validated {
            true => Ok(user.clone()),
            false => match PasswordUtils::compare_hash(
                validation_code.clone(),
                user.clone().validation_code.unwrap(),
            ) {
                false => {
                    if user.validation_tries >= 2 {
                        let new_code = code::generate_reset_code();
                        let _ = self
                            .user_repository
                            .update_validation_code(&new_code, &user.id);
                        MailService::send_validation_code(&user.email, &user.login, &new_code)
                            .await;
                        return Err(Status::TooManyRequests);
                    };
                    let _ = self
                        .user_repository
                        .update_validation_tries(&(user.validation_tries + 1), &user.id);
                    Err(Status::Unauthorized)
                }
                true => match self.user_repository.validate_account(&user.id) {
                    Err(_) => Err(Status::BadRequest),
                    Ok(user) => Ok(user),
                },
            },
        }
    }

    pub async fn send_reset_code(&self, login: &String, app_id: i32) {
        let user = match self.user_repository.get_by_login_and_app(login, app_id) {
            None => return,
            Some(user) => user,
        };
        let new_code = code::generate_reset_code();
        let _ = self.user_repository.update_forget_code(&new_code, &user.id);
        MailService::send_forget_code(&user.email, &user.login, &new_code).await;
    }

    pub fn reset_user_code(
        &self,
        login: &String,
        app_id: i32,
        validation_code: String,
        password: String,
    ) -> Result<User, Status> {
        let user_repository = UserRepository::new(&self.pool);
        let user = match user_repository.get_by_login_and_app(login, app_id) {
            None => return Err(Status::Unauthorized),
            Some(user) => user,
        };
        let u = user.clone();
        if user.validation_tries >= 2 {
            return Err(Status::TooManyRequests);
        }
        let user_code = match user.validation_code {
            None => {
                let _ =
                    user_repository.update_validation_tries(&(user.validation_tries + 1), &user.id);
                return Err(Status::NotFound);
            }
            Some(user_code) => user_code,
        };
        match PasswordUtils::compare_hash(validation_code, user_code) {
            false => {
                let _ =
                    user_repository.update_validation_tries(&(user.validation_tries + 1), &user.id);
                return Err(Status::Unauthorized);
            }
            true => {}
        };
        let delay = match user.forget_code_delay {
            None => {
                let _ =
                    user_repository.update_validation_tries(&(user.validation_tries + 1), &user.id);
                return Err(Status::Unauthorized);
            }
            Some(delay) => delay,
        };
        match delay > Utc::now() {
            false => Err(Status::RequestTimeout),
            true => {
                let hash_password = PasswordUtils::hash_password(password);
                let _ = user_repository.update_user_password(&user.id, hash_password);
                Ok(u)
            }
        }
    }

    pub async fn create_user(
        &self,
        input: UserInput,
        user_from: User,
        is_super_admin: bool,
    ) -> Result<User, Status> {
        // verification si la license permet la creation de l'utilisateur
        match self.allow_creation() {
            Err(status) => Err(status),
            Ok(_) => {
                let application_repository = ApplicationRepository::new(&self.pool);
                if is_super_admin && !input.is_admin {
                    return Err(Status::Unauthorized);
                };
                let application = match application_repository.get_by_id(input.application_id) {
                    None => return Err(Status::NotFound),
                    Some(app) => app,
                };
                if !is_super_admin && user_from.application != Some(application.id) {
                    return Err(Status::Unauthorized);
                };
                if !self.check_if_unique_login(&input.login, &application.id) {
                    return Err(Status::Conflict);
                };
                let insertable = UserInsertable::new(input, application).await;
                let user = self.user_repository.create_user(insertable);
                Ok(user)
            }
        }
    }

    pub fn update_password(
        &self,
        user_to_uuid: Uuid,
        user_from: User,
        password: String,
        is_admin: bool,
    ) -> Result<User, Status> {
        let user_to = match self.user_repository.get_by_id(&user_to_uuid) {
            None => return Err(Status::NotFound),
            Some(user_to) => user_to,
        };
        match is_admin {
            false => {
                if user_to.id != user_from.id {
                    return Err(Status::Unauthorized);
                }
            }
            true => {
                if user_to.application != user_from.application {
                    return Err(Status::Unauthorized);
                }
            }
        };
        let hash_password = PasswordUtils::hash_password(password);
        let _ = self
            .user_repository
            .update_user_password(&user_to_uuid, hash_password);
        Ok(user_to)
        // si pas d'erreur modifier le mot de passe
    }

    pub fn delete_user(
        &self,
        user_uuid: Uuid,
        user_from: User,
        is_super_admin: bool,
    ) -> Result<(), Status> {
        if !is_super_admin && user_uuid != user_from.id {
            return Err(Status::Unauthorized);
        }
        let _ = self.user_repository.delete_user(&user_uuid, &user_from.id);
        if !is_super_admin {
            let _ = self
                .application_repository
                .decrement_users(&user_from.application.unwrap());
        }
        Ok(())
    }

    pub fn get_totp_code(&self, user: &User) -> Result<UserTotpCode, Status> {
        let application = self
            .application_repository
            .get_by_id(user.application.unwrap())
            .unwrap();
        let new_totp = code::generate_base32_key(160);
        let user = self.user_repository.update_totp(&user.id, new_totp);
        Ok(UserTotpCode::new(user, application))
    }

    pub fn activate_2fa(
        &self,
        code: String,
        secret_key: String,
        user: User,
        activate: bool,
    ) -> Result<(), Status> {
        let code = code.parse::<u32>().unwrap();
        match code::check_otp(code, secret_key) {
            false => Err(Status::BadRequest),
            true => {
                let _ = self.user_repository.activate_2fa(&user.id, activate);
                Ok(())
            }
        }
    }

    pub async fn check_otp(
        &self,
        user: User,
        otp: Option<String>,
        ip: String,
        fingerprint: String,
        user_agent: String,
    ) -> Result<(), Status> {
        let ip_connexion = self.connexion_repository.get_user_ip_connexion(&user, &ip);
        let fingerprint_connexion = self
            .connexion_repository
            .get_user_fingerprint_connexion(&user, &fingerprint);
        if ip_connexion.is_none() || fingerprint_connexion.is_none() {
            match otp {
                None => return Err(Status::BadRequest),
                Some(totp) => {
                    let totp = match totp.parse::<u32>() {
                        Err(_) => 0,
                        Ok(totp) => totp,
                    };
                    if code::check_otp(totp, user.twofa_code) {
                        MailService::send_unfamiliar_connexion(
                            &user.email,
                            &user.login,
                            &ip,
                            &user_agent,
                        )
                        .await;
                        return Ok(());
                    };
                    return Err(Status::Unauthorized);
                }
            }
        }
        Ok(())
    }

    pub async fn check_new_connexion(
        &self,
        user: User,
        ip: String,
        fingerprint: String,
        user_agent: String,
    ) {
        let ip_connexion = self.connexion_repository.get_user_ip_connexion(&user, &ip);
        let fingerprint_connexion = self
            .connexion_repository
            .get_user_fingerprint_connexion(&user, &fingerprint);
        if ip_connexion.is_none() || fingerprint_connexion.is_none() {
            MailService::send_unfamiliar_connexion(&user.email, &user.login, &ip, &user_agent)
                .await;
        }
    }
}
