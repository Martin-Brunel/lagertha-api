use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::core::errors::{CustomError, ErrorObject};
use crate::dto::sentinel::sentinel_output::SentinelOutput;
use crate::dto::system::system_information_output::SystemInformationOutput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::services::sentinel::SentinelService;
use crate::services::system::SystemService;
use crate::traits::application::ApplicationContract;
use crate::{db::connect::DbPool, dto::system::system_version_dto::SystemVersionOutput};

use super::oauth::authorize;

/// # Retrieve the current system version, API name, and license information
///
/// Users can get the current version, name, API mode, license number,
/// license expiration date, and license name of the Lagertha_API system.
///
/// ## Roles
///
/// - `PUBLIC`
///
#[openapi(tag = "System")]
#[get("/system/version")]
pub async fn get_version() -> Json<SystemVersionOutput> {
    Json(SystemVersionOutput::new())
}

/// # Retrieve the current system statistics
///
/// Users can get the current number of users, applications, sentinels,
/// and anonymous sentinels in the Lagertha_API system.
///
/// ## Roles
///
/// - `ROLE_SUPER_ADMIN`
///
#[openapi(tag = "System")]
#[get("/system/informations")]
pub async fn get_system_informations(
    authorised: Security,
    pool: &rocket::State<DbPool>,
) -> Result<Json<SystemInformationOutput>, CustomError> {
    match authorised.check_roles(Role::SUPERADMIN) {
        false =>  Err(ErrorObject::create(Status::Unauthorized, None)),
        true => {
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let system_service = SystemService::new(pool, application_repository);
            let output = system_service.get_informations();

           Ok(Json(output))
        }
    }
}
