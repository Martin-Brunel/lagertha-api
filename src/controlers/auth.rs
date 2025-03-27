use rocket::post;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use std::net::SocketAddr;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;
use crate::dto::auth::auth_refresh_input::AuthRefreshInput;
use crate::dto::auth::{auth_input::AuthInput, auth_output::AuthOutput};
use crate::guards::user_agent::UserAgent;
use crate::repositories::application::ApplicationRepository;
use crate::repositories::connexion::ConnexionRepository;
use crate::repositories::revoked_token::RevokedTokenRepository;
use crate::services::application::ApplicationService;
use crate::services::auth::AuthService;
use crate::services::connexion::ConnexionService;
use crate::services::licence::LicenceService;
use crate::services::user::UserService;
use crate::traits::application::ApplicationContract;
use crate::traits::connexion::ConnexionContract;
use crate::traits::revoked_token::RevokedTokenContract;
use crate::LICENSE_VALID;

/// # User Authentication Endpoint
///
/// Authenticates a user and returns an `access_token` ,`refresh_token` and `open_id`.
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `login`: A string representing the user's login credentials.
///
/// - `password`:  A string representing the user's password.
///
/// - `application_id`:  A i32 representing the user's application id.
///
/// - `fingerprint`: A string representing the user's unique device identifier.
///
/// - `code_2fa`: An optional string representing the 6-digit Time-based One-Time Password (TOTP) if two-factor authentication is enabled.
///
#[openapi(tag = "Auth", ignore = "user_agent_guard")]
#[post("/auth", format = "json", data = "<auth_input>")]
pub async fn login(
    user_agent_guard: UserAgent,
    pool: &rocket::State<DbPool>,
    addr: SocketAddr,
    auth_input: Json<AuthInput>,
) -> Result<Json<AuthOutput>, CustomError> {
    let input = auth_input.into_inner();
    let pool = pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let app_service = ApplicationService::new(application_repository);
    match app_service
        .get_application_by_id(input.application_id)
        .await
    {
        Err(status) => Err(Custom(
            status,
            Json(ErrorObject::new(
                format!("Failed to get application"),
                status.code,
            )),
        )),
        Ok(app) => {
            let revoked_repository: RevokedTokenRepository =  RevokedTokenContract::new(&pool);
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let connexion_service = ConnexionService::new(connexion_repository);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let user_service = UserService::new(&pool, application_repository, connexion_repository);
            let auth_service = AuthService::new(&pool, revoked_repository, user_service, connexion_service);
            match auth_service
                .check_creds(&input.login, &input.password, &app)
                .await
            {
                Err(custom) => Err(custom),
                Ok(user) => {
                    let ip = addr.ip().to_string();
                    let user_agent = user_agent_guard.user_agent.unwrap_or_else(|| format!(""));
                    match auth_service
                        .check_otp(&user, input.code_2fa, &ip, &input.fingerprint, &user_agent)
                        .await
                    {
                        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
                        Ok(()) => {
                            let creds = auth_service
                                .generate_creds(&user, &app, &input.fingerprint, &user_agent)
                                .await;

                            let license = LicenceService::new().await.is_valid();
                            {
                                let mut license_valid = LICENSE_VALID.lock().unwrap();
                                *license_valid = license;
                            }

                            Ok(Json(creds))
                        }
                    }
                }
            }
        }
    }
}

/// # User Refresh Authentication Endpoint
///
/// Refresh user Authentication and returns an `access_token` ,`refresh_token` and `open_id`.
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `refresh_token`: A string representing the user's refresh token.
///
/// - `application_id`:  A i32 representing the user's application id.
///
/// - `fingerprint`: A string representing the user's unique device identifier.
///
#[openapi(tag = "Auth", ignore = "user_agent_guard")]
#[post("/auth/refresh", format = "json", data = "<auth_refresh_input>")]
pub async fn refresh(
    user_agent_guard: UserAgent,
    pool: &rocket::State<DbPool>,
    addr: SocketAddr,
    auth_refresh_input: Json<AuthRefreshInput>,
) -> Result<Json<AuthOutput>, CustomError> {
    let input = auth_refresh_input.into_inner();
    let pool = pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let app_service = ApplicationService::new(application_repository);
    match app_service
        .get_application_by_id(input.application_id)
        .await
    {
        Err(status) => Err(Custom(
            status,
            Json(ErrorObject::new(
                format!("Failed to get application"),
                status.code,
            )),
        )),
        Ok(app) => {
            let revoked_repository: RevokedTokenRepository =  RevokedTokenContract::new(&pool);
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let connexion_service = ConnexionService::new(connexion_repository);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let user_service = UserService::new(&pool, application_repository, connexion_repository);
            let auth_service = AuthService::new(&pool, revoked_repository, user_service, connexion_service);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let connexion_service = ConnexionService::new(connexion_repository);

            match auth_service.check_refresh_token(&input.refresh_token, &app) {
                Err(custom) => Err(custom),
                Ok(user) => {
                    let ip = addr.ip().to_string();
                    let user_agent = user_agent_guard.user_agent.unwrap();
                    connexion_service.create_connexion(
                        &ip,
                        &user_agent,
                        &input.fingerprint,
                        &user,
                    );
                    let creds = auth_service
                        .generate_creds(&user, &app, &input.fingerprint, &user_agent)
                        .await;
                    auth_service.revoke_token(&input.refresh_token);
                    Ok(Json(creds))
                }
            }
        }
    }
}
