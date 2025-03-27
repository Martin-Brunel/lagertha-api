use crate::core::nodes_config::NodesConfig;
use crate::dto::anonymous_sentinel::anonymous_sentinel_output::AnonymousSentinelOutput;
use crate::dto::anonymous_sentinel::anonymous_sentinel_public_input::AnonymousSentinelPublicInput;
use crate::dto::anonymous_sentinel::anonymous_sentinel_public_output::AnonymousSentinelPublicOutput;
use crate::dto::sentinel::sentinel_input::SentinelInput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::services::anonymous_sentinel::AnonymousSentinelService;
use crate::services::sentinel_log::SentinelLogService;
use crate::traits::application::ApplicationContract;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::tokio::spawn;
use rocket_okapi::openapi;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;

/// # Create a New Anonymous Sentinel
///
/// Allows users with `ROLE_USER` to create a new Anonymous Sentinel. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `clusters`: An array of strings representing the UUIDs of the clusters to be added.
///
#[openapi(tag = "Anonymous_Sentinels")]
#[post("/anonymous_sentinels", format = "json", data = "<sentinel_input>")]
pub async fn create(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    sentinel_input: Json<SentinelInput>,
) -> Result<Json<AnonymousSentinelOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let input = sentinel_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = AnonymousSentinelService::new(&pool,application_repository, &nodes_config);
    match authorised.check_roles(Role::USER) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => match sentinel_service.create(input, authorised.user) {
            Ok((sentinel, cipher)) => Ok(Json(AnonymousSentinelOutput::new(sentinel, cipher))),
            Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        },
    }
}

/// # Create a New Anonymous Sentinel (public)
///
/// Allows public users to create a new Anonymous Sentinel.
///
/// ## Roles
///
/// - `PUBLIC`
///
#[openapi(tag = "Anonymous_Sentinels")]
#[post(
    "/anonymous_sentinels/public",
    format = "json",
    data = "<sentinel_public_input>"
)]
pub async fn create_public(
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    sentinel_public_input: Json<AnonymousSentinelPublicInput>,
) -> Result<Json<AnonymousSentinelPublicOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let input = sentinel_public_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = AnonymousSentinelService::new(&pool,application_repository, &nodes_config);
    match sentinel_service.create_public(input) {
        Ok(sentinel) => Ok(Json(AnonymousSentinelPublicOutput::new(sentinel))),
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
    }
}

/// # Get Anonymous Sentinel public key
///
/// This Endpoint allow to retreive a the public key of the anonymous sentinel by its ID
///
/// ## Roles
///
/// - `PUBLIC`
///
/// ## Parameters
///
/// - `anonymous_sentinel_id`: A string representing the UUID of the Anonymous sentinel to be retrieved.
///
#[openapi(tag = "Anonymous_Sentinels")]
#[get("/anonymous_sentinels/public/<anonymous_sentinel_id>")]
pub async fn get_public_by_id(
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    anonymous_sentinel_id: &str,
) -> Result<Json<AnonymousSentinelPublicOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = AnonymousSentinelService::new(&pool,application_repository, &nodes_config);
    let sentinel_uuid = match Uuid::parse_str(&anonymous_sentinel_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Sentinel uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    match sentinel_service.get_public(sentinel_uuid) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(anonymous_sentinel) => Ok(Json(AnonymousSentinelPublicOutput::new(anonymous_sentinel))),
    }
}

/// # Get Anonymous Sentinel
///
/// Allows users with `ROLE_USER` to retrieve a Anonymous sentinel by its ID. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `anonymous_sentinel_id`: A string representing the UUID of the Anonymous sentinel to be retrieved.
///
#[openapi(tag = "Anonymous_Sentinels")]
#[get("/anonymous_sentinels/<anonymous_sentinel_id>")]
pub async fn get_by_id(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    anonymous_sentinel_id: &str,
    addr: SocketAddr,
) -> Result<Json<AnonymousSentinelOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = AnonymousSentinelService::new(&pool,application_repository, &nodes_config);
    let sentinel_uuid = match Uuid::parse_str(&anonymous_sentinel_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Sentinel uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    match authorised.check_roles(Role::USER) {
        false => {
            let anonymous_sentinel_id = anonymous_sentinel_id.to_string();
            spawn(async move {
                let _ = SentinelLogService::new_sentinel_log(
                    anonymous_sentinel_id,
                    &authorised.user,
                    false,
                    &addr.ip().to_string(),
                )
                .await;
            });
            Err(ErrorObject::create(Status::Unauthorized, None))
        }
        true => match sentinel_service.get_by_id(sentinel_uuid, authorised.user.clone()) {
            Err((status, msg)) => {
                let sentinel_uuid = sentinel_uuid.to_string();
                spawn(async move {
                    let _ = SentinelLogService::new_sentinel_log(
                        sentinel_uuid,
                        &authorised.user,
                        false,
                        &addr.ip().to_string(),
                    )
                    .await;
                });
                Err(ErrorObject::create(status, msg))
            }
            Ok((anonymous_sentinel, secret_key)) => {
                let sentinel_uuid = sentinel_uuid.to_string();
                spawn(async move {
                    let _ = SentinelLogService::new_sentinel_log(
                        sentinel_uuid,
                        &authorised.user,
                        true,
                        &addr.ip().to_string(),
                    )
                    .await;
                });
                let res = Json(AnonymousSentinelOutput::new(anonymous_sentinel, secret_key));

                Ok(res)
            }
        },
    }
}

/// # Delete an Anonymous Sentinel
///
/// Allows users with `ROLE_USER` to delete an Anonymous sentinel. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `anonymous_sentinel_id`: A string representing the UUID of the Anonymous sentinel to be deleted.
///
#[openapi(tag = "Anonymous_Sentinels")]
#[delete("/anonymous_sentinels/<anonymous_sentinel_id>")]
pub async fn delete_by_id(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    anonymous_sentinel_id: &str,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = AnonymousSentinelService::new(&pool,application_repository, &nodes_config);
    let sentinel_uuid = match Uuid::parse_str(&anonymous_sentinel_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Sentinel uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    match authorised.check_roles(Role::USER) || authorised.check_roles(Role::ADMIN) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => match sentinel_service.delete_one(
            sentinel_uuid,
            authorised.clone().user,
            authorised.check_roles(Role::ADMIN),
        ) {
            Ok(()) => Ok(Status::NoContent),
            Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        },
    }
}
