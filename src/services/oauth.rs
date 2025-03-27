use rocket::http::Status;
use uuid::Uuid;

use crate::{
    db::connect::DbPool,
    dto::{auth::auth_output::AuthOutput, oauth::oauth_token_input::OauthTokenInput},
    models::user::User,
    repositories::user::UserRepository,
    traits::{
        application::ApplicationContract, connexion::ConnexionContract,
        revoked_token::RevokedTokenContract,
    },
    utils::oauth::Oauth,
};

use super::auth::AuthService;

pub struct OauthService<T, D, C> {
    user_repository: UserRepository,
    application_repository: D,
    auth_service: AuthService<T, D, C>,
}

impl<T: RevokedTokenContract, D: ApplicationContract, C: ConnexionContract> OauthService<T, D, C> {
    pub fn new(
        pool: &DbPool,
        application_repository: D,
        auth_service: AuthService<T, D, C>,
    ) -> Self {
        Self {
            user_repository: UserRepository::new(pool),
            application_repository,
            auth_service,
        }
    }

    pub fn authorize(
        &self,
        user_from: User,
        user_id: Uuid,
        state: String,
    ) -> Result<String, (Status, Option<&str>)> {
        let user = match self.user_repository.get_by_id(&user_id) {
            None => return Err((Status::NotFound, Some("User not found"))),
            Some(user) => user,
        };
        match user_from.application == user.application {
            false => Err((Status::Forbidden, None)),
            true => {
                let authorize_code = Oauth::create(user, state);
                Ok(authorize_code)
            }
        }
    }

    pub async fn check(
        &self,
        input: OauthTokenInput,
        user_agent: String,
        ip: String,
    ) -> Result<AuthOutput, (Status, Option<&str>)> {
        match Oauth::check(input.authorization_code, input.state).await {
            Err(e) => Err(e),
            Ok(oauth) => {
                let user = match self.user_repository.get_by_id(&oauth.id) {
                    None => return Err((Status::NotFound, None)),
                    Some(user) => user,
                };
                let application = match self
                    .application_repository
                    .get_by_id(user.application.unwrap().clone())
                {
                    None => return Err((Status::NotFound, None)),
                    Some(application) => application,
                };
                match self
                    .auth_service
                    .check_otp(&user, input.code_2fa, &ip, &input.fingerprint, &user_agent)
                    .await
                {
                    Err((status, msg)) => Err((status, msg)),
                    Ok(()) => {
                        let creds = self
                            .auth_service
                            .generate_creds(&user, &application, &input.fingerprint, &user_agent)
                            .await;
                        Ok(creds)
                    }
                }
            }
        }
    }
}
