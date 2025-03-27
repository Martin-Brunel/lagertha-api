use rocket::post;
use rocket::{http::Status, serde::json::Json};
use rocket_okapi::openapi;
use uuid::Uuid;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;
use crate::dto::user::user_2fa_activate_input::User2faActivateInput;
use crate::dto::user::user_forget_input::UserForgetInput;
use crate::dto::user::user_input::UserInput;
use crate::dto::user::user_output::UserOutput;
use crate::dto::user::user_password_input::UserPasswordInput;
use crate::dto::user::user_public_input::UserPublicInput;
use crate::dto::user::user_reset_input::UserResetInput;
use crate::dto::user::user_totp_code::UserTotpCode;
use crate::dto::user::user_validation_input::UserValidationInput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::repositories::connexion::ConnexionRepository;
use crate::services::user::UserService;
use crate::traits::application::ApplicationContract;
use crate::traits::connexion::ConnexionContract;

/// # Create a new user (public)
///
/// This endpoint allows the creation of a new user with the role `ROLE_USER`.
///
/// This endpoint is publicly accessible and does not require any authentication.
///
/// The created user (ROLE_VALIDATION) must validate their account to end the process
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `email`: A String representing the email of the user
///
/// - `password`: A String representing the strong user  password (must be min length 8, at least 1 lowercase, least 1 uppercase, at least 1 number )
///  
/// - `firstname`: A String representing the user firstname
///
/// - `lastname`:  A String representing the user lastname
///
/// - `login`: A String representing the user login (must be unique for the givent application)
///
/// - `application_id`: A number representing the application id
///
#[openapi(tag = "Users")]
#[post("/users/public", format = "json", data = "<user_public_input>")]
pub async fn post_user_public(
    pool: &rocket::State<DbPool>,
    user_public_input: Json<UserPublicInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_public_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    match user_service.create_user_public(input).await {
        Err(status) => Err(ErrorObject::create(status, None)),
        Ok(user) => Ok(Json(UserOutput::new(user))),
    }
}

/// #  Create a new user (admin, super admin)
///
/// This endpoint allows the creation of a new user with the role `ROLE_USER` or `ROLE_ADMIN`.
///
/// This endpoint require to be authenticate.
///
/// ## Roles
///
/// - `ROLE_SUPER_ADMIN`
///
/// - `ROLE_ADMIN`
///
/// ## Parameters
///
/// - `email`: A String representing the email of the user
///
/// - `password`: An Optional String representing the strong user password (must be min length 8, at least 1 lowercase, least 1 uppercase, at least 1 number )
///  
/// - `firstname`: A String representing the user firstname
///
/// - `lastname`:  A String representing the user lastname
///
/// - `login`: A String representing the user login (must be unique for the givent application)
///
/// - `application_id`: A number representing the application id (ignore if ROLE_ADMIN)
///
/// - `is_admin`: A boolean representing the role of the future user (true -> ROLE_ADMIN, false -> ROLE_USER)
///
#[openapi(tag = "Users")]
#[post("/users", format = "json", data = "<user_input>")]
pub async fn post_user(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    user_input: Json<UserInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);

    match authorised.check_roles(Role::SUPERADMIN) || authorised.check_roles(Role::ADMIN) {
        false => Err(ErrorObject::create(Status::Forbidden, None)),
        true => match user_service
            .create_user(
                input,
                authorised.clone().user,
                authorised.check_roles(Role::SUPERADMIN),
            )
            .await
        {
            Err(status) => Err(ErrorObject::create(status, None)),
            Ok(user) => Ok(Json(UserOutput::new(user))),
        },
    }
}

/// # Validate user account
///
/// This endpoint allows users with the `ROLE_VALIDATION` permission to validate a user account by providing a validation code.
///
/// This endpoint requires authentication and the `ROLE_VALIDATION` role. It accepts a validation code to validate a user account.
///
/// If the validation is successful, it returns basic information about the user.
///
/// ## Roles
///
/// - `ROLE_VALIDATION`
///
/// ## Parameters
///
/// - `validation_code`: A string representing the validation code.
///
#[openapi(tag = "Users")]
#[post(
    "/users/validate",
    format = "application/json",
    data = "<user_validation_input>"
)]
pub async fn validate_user(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    user_validation_input: Json<UserValidationInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_validation_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    match authorised.check_roles(Role::VALIDATION) {
        false => Err(ErrorObject::create(
            Status::Forbidden,
            Some("Only user with ROLE_VALIDATION role"),
        )),
        true => match user_service
            .validate_user(&authorised.user, &input.validation_code)
            .await
        {
            Err(status) => {
                let error_message = match status.code {
                    401 => "Wrong code",
                    429 => "Too many tries",
                    _ => status.reason().unwrap_or_else(|| ""),
                };
                Err(ErrorObject::create(status, Some(error_message)))
            }
            Ok(user) => Ok(Json(UserOutput::new(user))),
        },
    }
}

/// # Request a Password Reset Code
///
/// This endpoint allows users to request a temporary validation code for resetting their password.
///
/// This endpoint is publicly accessible and does not require authentication.
///
/// It accepts the user's login and application ID to send a temporary validation code to the user's registered email address for password reset purposes.
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `login`: A string representing the user's login.
///
/// - `application_id`: A string representing the application ID.
///
#[openapi(tag = "Users")]
#[post(
    "/users/forget",
    format = "application/json",
    data = "<user_forget_input>"
)]
pub async fn forget_user_password(
    pool: &rocket::State<DbPool>,
    user_forget_input: Json<UserForgetInput>,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_forget_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    user_service
        .send_reset_code(&input.login, input.application_id)
        .await;
    Ok(Status::Created)
}

/// # Reset Password
///
/// This endpoint allows users to reset their password using a temporary validation code.
///
/// This endpoint is publicly accessible and does not require authentication.
///
/// It accepts the user's login, application ID,
/// validation code, and the new password to reset the user's password.
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `login`: A string representing the user's login.
///
/// - `application_id`: A string representing the application ID.
///
/// - `code`: A string representing the validation code.
///
/// - `new_password`: A string representing the new password.
///
#[openapi(tag = "Users")]
#[post(
    "/users/reset_password",
    format = "application/json",
    data = "<user_reset_input>"
)]
pub async fn reset_user_password(
    pool: &rocket::State<DbPool>,
    user_reset_input: Json<UserResetInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_reset_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    match user_service.reset_user_code(
        &input.login,
        input.application_id,
        input.code,
        input.new_password,
    ) {
        Err(status) => Err(ErrorObject::create(status, None)),
        Ok(user) => Ok(Json(UserOutput::new(user))),
    }
}

/// # Update User Password
///
/// This endpoint allows users with the roles `ROLE_ADMIN`, `ROLE_SUPER_ADMIN`, or `ROLE_USER` to update their password.
///
/// This endpoint requires authentication.
///
/// Admin users with `ROLE_ADMIN`  can update passwords for users with
/// the role `ROLE_USER` within their applications.
///
/// Users with `ROLE_USER` or `ROLE_SUPER_ADMIN` can update their own passwords.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_SUPER_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `user_id`: A string representing the UUID of the user whose password is being updated.
///     
/// - `password`: A string representing the new password.
///
#[openapi(tag = "Users")]
#[put("/users/password", format = "json", data = "<user_password>")]
pub async fn update_user_password(
    pool: &rocket::State<DbPool>,
    authorised: Security,
    user_password: Json<UserPasswordInput>,
) -> Result<Json<UserOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = user_password.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    let is_admin = authorised.check_roles(Role::ADMIN);
    let user_from = authorised.user;
    let user_to_id = input.user_id;

    let user_to_uuid = match Uuid::parse_str(&user_to_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad user uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    match user_service.update_password(user_to_uuid, user_from, input.password, is_admin) {
        Err(status) => Err(ErrorObject::create(status, None)),
        Ok(user) => Ok(Json(UserOutput::new(user))),
    }
}

/// # Delete User
///
/// This endpoint allows users with the `ROLE_SUPER_ADMIN` permission to delete any user, while all other users are only authorized
/// to delete their own accounts.
///
/// Users with the `ROLE_SUPER_ADMIN` role can delete any user account.
///
/// Users with `ROLE_ADMIN` or `ROLE_USER` roles can only delete
/// their own accounts.
///
/// The endpoint accepts the user's ID and performs the deletion operation.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_SUPER_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `user_id`: A string representing the UUID of the user to be deleted.
///
#[openapi(tag = "Users")]
#[delete("/users/<user_id>")]
pub async fn delete_user(
    pool: &rocket::State<DbPool>,
    authorised: Security,
    user_id: String,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    let is_super_admin = authorised.check_roles(Role::SUPERADMIN);
    let user_to_uuid = match Uuid::parse_str(&user_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad user uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let user_from = authorised.user;
    match user_service.delete_user(user_to_uuid, user_from, is_super_admin) {
        Err(status) => Err(ErrorObject::create(status, None)),
        Ok(_) => Ok(Status::NoContent),
    }
}

/// # Get 2FA Code
///
/// If 2FA is not enabled, this endpoint generates and returns a unique url to activate 2FA.
///
/// This endpoint requires authentication. It checks if 2FA is not already activated for the user and, if not,
/// generates a unique Time-based One-Time Password (TOTP) url to activate 2FA.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_SUPER_ADMIN`
///
#[openapi(tag = "Users")]
#[get("/users/2fa/code")]
pub async fn get_2fa_code(
    authorised: Security,
    pool: &rocket::State<DbPool>,
) -> Result<Json<UserTotpCode>, CustomError> {
    let pool = pool.inner().to_owned();
    let user = authorised.user;
    match !user.is_2fa_activated {
        false => Err(ErrorObject::create(
            Status::Forbidden,
            Some("2fa still activated"),
        )),
        true => {
            let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
            let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
            let user_service =
                UserService::new(&pool, application_repository, connexion_repository);
            match user_service.get_totp_code(&user) {
                Err(status) => Err(ErrorObject::create(status, None)),
                Ok(totp) => Ok(Json(totp)),
            }
        }
    }
}

/// # Enable 2FA Authentication
///
/// This endpoint enables 2FA authentication on the user's account.
///
/// This endpoint requires authentication.
///
/// It accepts a TOTP code and a flag to activate or deactivate 2FA
/// for the user's account.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_SUPER_ADMIN`
///
/// ## Parameters
///
/// - `code`: A string representing the TOTP code.
///       
/// - `is_activate`: A boolean indicating whether to activate or deactivate 2FA.
///
#[openapi(tag = "Users")]
#[put(
    "/users/2fa/activate",
    format = "json",
    data = "<user_2fa_activate_input>"
)]
pub async fn activate_2fa(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    user_2fa_activate_input: Json<User2faActivateInput>,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let user_service = UserService::new(&pool, application_repository, connexion_repository);
    let secret_key = authorised.user.clone().twofa_code;
    let code = user_2fa_activate_input.clone().into_inner().code;
    let activate = user_2fa_activate_input.into_inner().is_activate;

    match user_service.activate_2fa(code, secret_key, authorised.user, activate) {
        Ok(_) => Ok(Status::Created),
        Err(status) => Err(ErrorObject::create(status, None)),
    }
}
