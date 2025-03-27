use crate::dto::oidc::OidcVerifyInput;
use crate::dto::user::user_output::UserOutput;
use crate::guards::security::Security;
use crate::services::oidc::OidcService;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;

/// # Verifies an OpenID Connect Token.
/// 
/// This endpoint verifies the validity of an OpenID Connect Token. It checks the signature and expiration of the token. 
/// If valid, it returns basic information about the user.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
/// 
/// ## Parameters
///
/// - `open_id_token`: A string representing the OpenID Connect Token to be verified.
/// 
#[openapi(tag = "Oidc")]
#[post("/oidc/verify", format = "json", data = "<oidc_verify_input>")]
pub async fn verify(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    oidc_verify_input: Json<OidcVerifyInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = oidc_verify_input.into_inner();
    let oidc_service = OidcService::new(&pool);
    match oidc_service.oidc_verify(&input.open_id_token, &authorised.user) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg.as_deref())),
        Ok(user) => Ok(Json(UserOutput::new(user))),
    }
}