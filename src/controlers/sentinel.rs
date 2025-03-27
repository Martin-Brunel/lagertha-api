use crate::core::nodes_config::NodesConfig;
use crate::dto::sentinel::sentinel_input::SentinelInput;
use crate::dto::sentinel::sentinel_output::SentinelOutput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::services::sentinel::SentinelService;
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

/// # Create a New Sentinel
///
/// Allows users with `ROLE_USER` to create a new sentinel. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `clusters`: An array of strings representing the UUIDs of the clusters to be added.
///
#[openapi(tag = "Sentinels")]
#[post("/sentinels", format = "json", data = "<sentinel_input>")]
pub async fn create(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    sentinel_input: Json<SentinelInput>,
) -> Result<Json<SentinelOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let input = sentinel_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = SentinelService::new(&pool, application_repository, &nodes_config);
    match authorised.check_roles(Role::USER) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => match sentinel_service.create(input, authorised.user) {
            Ok((sentinel, cipher)) => Ok(Json(SentinelOutput::new(sentinel, cipher))),
            Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        },
    }
}

/// # Get Sentinel
///
/// Allows users with `ROLE_USER` to retrieve a sentinel by its ID. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `sentinel_id`: A string representing the UUID of the sentinel to be retrieved.
///
#[openapi(tag = "Sentinels")]
#[get("/sentinels/<sentinel_id>")]
pub async fn get_by_id(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    sentinel_id: &str,
    addr: SocketAddr,
) -> Result<Json<SentinelOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = SentinelService::new(&pool, application_repository, &nodes_config);
    let sentinel_uuid = match Uuid::parse_str(&sentinel_id) {
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
            let sentinel_id = sentinel_id.to_string();
            spawn(async move {
                let _ = SentinelLogService::new_sentinel_log(
                    sentinel_id,
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
                let sentinel_id = sentinel_id.to_string();
                spawn(async move {
                    let _ = SentinelLogService::new_sentinel_log(
                        sentinel_id,
                        &authorised.user,
                        false,
                        &addr.ip().to_string(),
                    )
                    .await;
                });
                Err(ErrorObject::create(status, msg))
            }
            Ok((sentinel, cipher)) => {
                let sentinel_id = sentinel_id.to_string();
                spawn(async move {
                    let _ = SentinelLogService::new_sentinel_log(
                        sentinel_id,
                        &authorised.user,
                        true,
                        &addr.ip().to_string(),
                    )
                    .await;
                });
                let res = Json(SentinelOutput::new(sentinel, cipher));

                Ok(res)
            }
        },
    }
}

/// # Delete a Sentinel
///
/// Allows users with `ROLE_USER` to delete a sentinel. The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `sentinel_id`: A string representing the UUID of the sentinel to be deleted.
///
#[openapi(tag = "Sentinels")]
#[delete("/sentinels/<sentinel_id>")]
pub async fn delete_by_id(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    nodes_config: &rocket::State<NodesConfig>,
    sentinel_id: &str,
) -> Result<Status, CustomError> {
    let pool = pool.inner().to_owned();
    let nodes_config = nodes_config.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let sentinel_service = SentinelService::new(&pool,application_repository, &nodes_config);
    let sentinel_uuid = match Uuid::parse_str(&sentinel_id) {
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
