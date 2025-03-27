use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use sha2::{Digest, Sha256};

use crate::{
    core::errors::{CustomError, ErrorObject},
    db::connect::DbPool,
    dto::{
        auth::auth_output::AuthOutput,
        revoked_token::revoked_token_insertable::RevokedTokenInsertable,
    },
    models::{application::Application, revoked_token::RevokedToken, user::User},
    repositories::{revoked_token::RevokedTokenRepository, user::UserRepository},
    traits::{
        application::ApplicationContract, connexion::ConnexionContract,
        revoked_token::RevokedTokenContract,
    },
    utils::{jwt::Jwt, open_id::OpenId},
};

use super::{connexion::ConnexionService, user::UserService};

pub struct AuthService<T, D, C> {
    user_service: UserService<D, C>,
    user_repository: UserRepository,
    revoked_repository: T,
    connexion_service: ConnexionService<C>,
}

impl<T: RevokedTokenContract, D: ApplicationContract, C: ConnexionContract> AuthService<T, D, C> {
    pub fn new(
        pool: &DbPool,
        revoked_repository: T,
        user_service: UserService<D, C>,
        connexion_service: ConnexionService<C>,
    ) -> Self {
        Self {
            user_service,
            user_repository: UserRepository::new(pool),
            revoked_repository,
            connexion_service,
        }
    }

    pub fn verify_password(password: &String, hash: &String) -> bool {
        match bcrypt::verify(password, hash) {
            Err(_) => false,
            Ok(res) => res,
        }
    }

    /// check the provided credentials
    pub async fn check_creds(
        &self,
        login: &String,
        password: &String,
        app: &Application,
    ) -> Result<User, CustomError> {
        let custom_error = Custom(
            Status::Unauthorized,
            Json(ErrorObject::new(
                format!("Unauthorized: Access is denied due to invalid credentials."),
                401,
            )),
        );
        match self.user_repository.get_by_login_and_app(login, app.id) {
            None => Err(custom_error),
            Some(user) => match user.clone().password.is_some()
                && Self::verify_password(password, &user.clone().password.unwrap())
            {
                false => Err(custom_error),
                true => Ok(user),
            },
        }
    }

    pub fn check_refresh_token(
        &self,
        refresh_token: &String,
        app: &Application,
    ) -> Result<User, CustomError> {
        let custom_error = Custom(
            Status::Unauthorized,
            Json(ErrorObject::new(
                format!("Unauthorized: Access is denied due to invalid credentials."),
                401,
            )),
        );
        let revoked_token = self.revoked_repository.get_by_token(refresh_token);
        match revoked_token {
            Some(_) => Err(custom_error),
            None => match decode::<Jwt>(
                refresh_token.as_ref(),
                &DecodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/public.key")).unwrap(),
                &Validation::new(Algorithm::RS256),
            ) {
                Err(_) => Err(custom_error),
                Ok(token) => {
                    if !token.claims.is_refresh {
                        return Err(custom_error);
                    };
                    match self
                        .user_repository
                        .get_by_login_and_app(&token.claims.login, app.id)
                    {
                        None => Err(custom_error),
                        Some(user) => Ok(user),
                    }
                }
            },
        }
    }

    pub fn revoke_token(&self, token: &String) -> RevokedToken {
        let insertable = RevokedTokenInsertable::new(token.clone());
        self.revoked_repository.create_revoked_token(insertable)
    }

    pub async fn generate_creds(
        &self,
        user: &User,
        application: &Application,
        fingerprint: &String,
        user_agent: &String,
    ) -> AuthOutput {
        let validation_check_user = user.validation();
        let to_hash = &format!("{}{}", fingerprint, user_agent);
        let mut hasher = Sha256::new();
        hasher.update(to_hash);
        let device_sha = format!("{:x}", hasher.finalize());
        let jwt =
            Jwt::create_jwt(&validation_check_user, false, application, Some(device_sha)).await;
        let refresh = Jwt::create_jwt(&validation_check_user, true, application, None).await;
        let open_id = OpenId::create(validation_check_user.clone()).await;
        AuthOutput::new(jwt, refresh, open_id).await
    }

    pub async fn check_otp(
        &self,
        user: &User,
        code_2fa: Option<String>,
        ip: &String,
        fingerprint: &String,
        user_agent: &String,
    ) -> Result<(), (Status, Option<&str>)> {
        if user.is_2fa_activated {
            match self
                .user_service
                .check_otp(
                    user.clone(),
                    code_2fa,
                    ip.clone(),
                    fingerprint.clone(),
                    user_agent.clone(),
                )
                .await
            {
                Err(status) => return Err((status, Some("Missing TOTP code"))),
                Ok(_) => {}
            }
        } else {
            self.user_service
                .check_new_connexion(
                    user.clone(),
                    ip.clone(),
                    fingerprint.clone(),
                    user_agent.clone(),
                )
                .await;
        }
        self.connexion_service
            .create_connexion(ip, user_agent, fingerprint, user);
        Ok(())
    }
}
