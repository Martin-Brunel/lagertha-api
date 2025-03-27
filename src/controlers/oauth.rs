use crate::dto::auth::auth_output::AuthOutput;
use crate::dto::oauth::oauth_authorize_output::OauthAuthorizeOutput;
use crate::dto::oauth::oauth_token_input::OauthTokenInput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::guards::user_agent::UserAgent;
use crate::repositories::application::ApplicationRepository;
use crate::repositories::connexion::ConnexionRepository;
use crate::repositories::revoked_token::RevokedTokenRepository;
use crate::services::auth::AuthService;
use crate::services::connexion::ConnexionService;
use crate::services::user::UserService;
use crate::traits::application::ApplicationContract;
use crate::traits::connexion::ConnexionContract;
use crate::traits::revoked_token::RevokedTokenContract;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;
use crate::services::oauth::OauthService;

/// # Requests a temporary authorization code for a user account.
///
/// Initiates the authorization process for a `ROLE_ADMIN` to obtain a temporary authorization code on behalf of a `ROLE_USER`.
///
/// This endpoint assumes that the `ROLE_ADMIN` is already authenticated. The `ROLE_ADMIN` specifies the user account for which the authorization is requested and provides a unique `state` for this session.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// ## Parameters
///
/// - `client_id`: A string representing the unique identifier (user id) of the user account for which the authorization is requested.
///
/// - `state`: A string representing a unique state value to maintain state between the request and callback.
///
#[openapi(tag = "Oauth")]
#[get("/oauth/authorize?<client_id>&<state>")]
pub async fn authorize(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    client_id: String,
    state: String,
) -> Result<Json<OauthAuthorizeOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    match authorised.check_roles(Role::ADMIN) {
        false => Err(ErrorObject::create(Status::Forbidden, None)),
        true => {
            let user_to_uuid = match Uuid::parse_str(&client_id) {
                Err(_) => {
                    return Err(ErrorObject::create(
                        Status::BadRequest,
                        Some("Bad user uuid"),
                    ))
                }
                Ok(uuid) => uuid,
            };
            let revoked_repository: RevokedTokenRepository = RevokedTokenContract::new(&pool);
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let user_service =
                UserService::new(&pool, application_repository, connexion_repository);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let connexion_service = ConnexionService::new(connexion_repository);
            let auth_service =
                AuthService::new(&pool, revoked_repository, user_service, connexion_service);
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let oauth_service = OauthService::new(&pool, application_repository, auth_service);

            match oauth_service.authorize(authorised.user, user_to_uuid, state) {
                Err((status, msg)) => Err(ErrorObject::create(status, msg)),
                Ok(authorize_code) => Ok(Json(OauthAuthorizeOutput::new(authorize_code))),
            }
        }
    }
}

/// # Exchanges authorization code for an access token.
///
/// This endpoint allows `ROLE_USER` to exchange the authorization code, obtained by `ROLE_ADMIN`, for an access token. The request must include the original state value for validation.
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `authorization_code`: A string given by the server to exchange to an lagertha access token .
///
/// - `state`: A string representing a unique state value to maintain state between the request and callback (provided by server).
///
/// - `fingerprint`: A unique device authentifier
///
/// - `code_2fa`: If MFA is acticated, provide the 6 number T-otp code
///
#[openapi(tag = "Oauth", ignore = "user_agent_guard")]
#[post("/oauth/token", format = "json", data = "<oauth_token_input>")]
pub async fn token(
    user_agent_guard: UserAgent,
    addr: SocketAddr,
    pool: &rocket::State<DbPool>,
    oauth_token_input: Json<OauthTokenInput>,
) -> Result<Json<AuthOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = oauth_token_input.into_inner();

    let revoked_repository: RevokedTokenRepository = RevokedTokenContract::new(&pool);
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let connexion_service = ConnexionService::new(connexion_repository);
    let auth_service = AuthService::new(&pool, revoked_repository, user_service, connexion_service);
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let oauth_service = OauthService::new(&pool, application_repository, auth_service);

    let ip = addr.ip().to_string();
    match oauth_service
        .check(
            input,
            user_agent_guard
                .user_agent
                .unwrap_or_else(|| String::from("")),
            ip,
        )
        .await
    {
        Err((status, msg)) => Err(ErrorObject::create(status, msg.as_deref())),
        Ok(creds) => Ok(Json(creds)),
    }
}
