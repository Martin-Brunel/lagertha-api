use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;
use crate::dto::application::application_input::ApplicationInput;
use crate::dto::application::application_output::ApplicationOutput;
use crate::dto::application::application_update_input::ApplicationUpdateInput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::services::application::ApplicationService;
use crate::traits::application::ApplicationContract;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// # Create a New Application
///
/// This endpoint allows users with the `ROLE_SUPER_ADMIN` permission to create a new application.
///
/// ## Roles
///
/// - `ROLE_SUPER_ADMIN`
///
/// ## Parameters
///
/// - `name`: A String representing the new application name.
///
/// - `contact_email`: A String representing the new application contact email
///
#[openapi(tag = "Applications")]
#[post("/applications", format = "json", data = "<application_input>")]
pub async fn post_application(
    pool: &rocket::State<DbPool>,
    authorised: Security,
    application_input: Json<ApplicationInput>,
) -> Result<Json<ApplicationOutput>, CustomError> {
    let pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>> =
        pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let application_services = ApplicationService::new(application_repository);
    match authorised.check_roles(Role::SUPERADMIN) {
        false => Err(ErrorObject::create(Status::Forbidden, None)),
        true => {
            let application =
                application_services.create_application(application_input.into_inner());
            let output = ApplicationOutput::new(application);
            Ok(Json(output))
        }
    }
}

/// # Delete Application
///
/// This endpoint allows users with the `ROLE_SUPER_ADMIN` permission to delete an existing application.
///
/// ## Roles
///
/// - `ROLE_SUPER_ADMIN`
///
/// ## Parameters
///
/// - `application_id`: The ID of the application to be deleted.
///
#[openapi(tag = "Applications")]
#[delete("/applications/<application_id>")]
pub async fn delete_application(
    pool: &rocket::State<DbPool>,
    authorised: Security,
    application_id: i32,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    match authorised.check_roles(Role::SUPERADMIN) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => {
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let application_services = ApplicationService::new(application_repository);
            let user_from = authorised.user;
            match application_services.delete_application(application_id, user_from) {
                Err(status) => Err(ErrorObject::create(status, None)),
                Ok(_) => Ok(Status::NoContent),
            }
        }
    }
}

/// # Update Application
///
/// This endpoint allows users with the `ROLE_SUPER_ADMIN` permission to update an existing application.
///
/// ## Roles
///
/// - `ROLE_SUPER_ADMIN`
///
/// ## Parameters
///
/// - `id`: A i32 representing the id of the target application.
///
/// - `name`: An optionnal String represanting the Updated Application Name
///
/// - `contact_email`: An optionnal String represanting the Updated Application contact email
///
#[openapi(tag = "Applications")]
#[put("/applications", format = "json", data = "<application_update_input>")]
pub async fn update_application(
    pool: &rocket::State<DbPool>,
    authorised: Security,
    application_update_input: Json<ApplicationUpdateInput>,
) -> Result<Json<ApplicationOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = application_update_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let application_services = ApplicationService::new(application_repository);
    match authorised.check_roles(Role::SUPERADMIN) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => {
            let user_from = authorised.user;
            match application_services.update_application(input, user_from) {
                Err(status) => Err(ErrorObject::create(status, None)),
                Ok(application) => Ok(Json(ApplicationOutput::new(application))),
            }
        }
    }
}
