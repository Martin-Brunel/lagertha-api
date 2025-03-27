use crate::dto::cluster::cluster_anonymous_sentinels_input::ClusterAnonymousSentinelsInput;
use crate::dto::cluster::cluster_input::ClusterInput;
use crate::dto::cluster::cluster_memberships_input::ClusterMembershipsInput;
use crate::dto::cluster::cluster_output::ClusterOutput;
use crate::dto::cluster::cluster_sentinels_input::ClusterSentinelsInput;
use crate::dto::list::ListDto;
use crate::dto::user::user_output::UserOutput;
use crate::enums::roles::Role;
use crate::guards::security::Security;
use crate::repositories::application::ApplicationRepository;
use crate::repositories::connexion::ConnexionRepository;
use crate::services::cluster::ClusterService;
use crate::traits::application::ApplicationContract;
use crate::traits::connexion::ConnexionContract;
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use uuid::Uuid;

use crate::core::errors::{CustomError, ErrorObject};
use crate::db::connect::DbPool;

/// # Create a New Cluster
///
/// Allows users with `ROLE_USER` or `ROLE_ADMIN` to create a new cluster.
///
/// The user must be authenticated and authorized to perform this action.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///   
/// - `name`: A string representing the name of the cluster.
///
/// - `description`: A Optional string providing a description of the cluster.
///
/// - `memberships`: An array of strings representing the UUIDs of the users to be added.
///
#[openapi(tag = "Clusters")]
#[post("/clusters", format = "json", data = "<cluster_input>")]
pub async fn create(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_input: Json<ClusterInput>,
) -> Result<Json<ClusterOutput>, CustomError> {
    let pool = pool.inner().to_owned();
    let input = cluster_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match authorised.check_roles(Role::ADMIN) || authorised.check_roles(Role::USER) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => match cluster_service.create(input, authorised.user) {
            Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
            Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        },
    }
}

/// # Add Members to a Cluster
///
/// Adds a list of users as members of the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application, and by `ROLE_USER` on any cluster they have created.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///
/// - `memberships`: An array of strings representing the UUIDs of the users to be added.
///
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/add_memberships",
    format = "json",
    data = "<cluster_memberships_input>"
)]
pub async fn add_memberships(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_memberships_input: Json<ClusterMembershipsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_memberships_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.add_memberships(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}

/// # Remove Members from a Cluster
///
/// Removes a list of users from the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application, and by `ROLE_USER` on any cluster they have created.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///
/// - `memberships`: An array of strings representing the UUIDs of the users to be removed.
///
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/remove_memberships",
    format = "json",
    data = "<cluster_memberships_input>"
)]
pub async fn remove_memberships(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_memberships_input: Json<ClusterMembershipsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_memberships_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.remove_memberships(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}

/// # Delete a Cluster
///
/// Deletes the specified cluster based on its unique identifier. Users with `ROLE_ADMIN` can delete any cluster within their application.
/// Users with `ROLE_USER` can delete clusters only if they are the creators of those clusters.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster to be deleted.
///
#[openapi(tag = "Clusters")]
#[delete("/clusters/<cluster_id>")]
pub async fn delete_cluster(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_id: &str,
) -> Result<Status, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.delete(cluster_uuid, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(_) => Ok(Status::NoContent),
    }
}

/// # Add Sentinels to a Cluster
///
/// Adds a list of Sentinels to the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application,
/// and by `ROLE_USER` on any cluster they have created.
///
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///
/// - `sentinels`: An array of strings representing the UUIDs of the sentinels to be added.
///
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/add_sentinels",
    format = "json",
    data = "<cluster_sentinels_input>"
)]
pub async fn add_sentinels(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_sentinels_input: Json<ClusterSentinelsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_sentinels_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.add_sentinels(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}
/// # Add Anonymous Sentinels to a Cluster
///
/// Adds a list of anonymous Sentinels as members of the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application,
/// and by `ROLE_USER` on any cluster they have created.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///
/// - `anonymous_sentinels`: An array of strings representing the UUIDs of the anonymous sentinels to be added.
///   
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/add_anonymous_sentinels",
    format = "json",
    data = "<cluster_sentinels_input>"
)]
pub async fn add_anonymous_sentinels(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_sentinels_input: Json<ClusterAnonymousSentinelsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_sentinels_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.add_anonymous_sentinels(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}

/// # Remove Sentinels from a Cluster
///
/// Removes a list of Sentinels from the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application, and by `ROLE_USER` on any cluster they have created.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///     
/// - `sentinel_ids`: An array of strings representing the UUIDs of the sentinels to be removed.
///
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/remove_sentinels",
    format = "json",
    data = "<cluster_sentinels_input>"
)]
pub async fn remove_sentinels(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_sentinels_input: Json<ClusterSentinelsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_sentinels_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.remove_sentinels(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}

/// # Remove Anonymous Sentinels from a Cluster
///
/// Removes a list of anonymous Sentinels from the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application, and by `ROLE_USER` on any cluster they have created.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// - `ROLE_USER`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///  
/// - `anonymous_sentinels`: An array of strings representing the UUIDs of the anonymous sentinels to be removed.
///
#[openapi(tag = "Clusters")]
#[put(
    "/clusters/<cluster_id>/remove_anonymous_sentinels",
    format = "json",
    data = "<cluster_sentinels_input>"
)]
pub async fn remove_anonymous_sentinels(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_sentinels_input: Json<ClusterAnonymousSentinelsInput>,
    cluster_id: &str,
) -> Result<Json<ClusterOutput>, CustomError> {
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let pool = pool.inner().to_owned();
    let input = cluster_sentinels_input.into_inner();
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);
    match cluster_service.remove_anonymous_sentinels(cluster_uuid, input, authorised) {
        Err((status, msg)) => Err(ErrorObject::create(status, msg)),
        Ok(cluster) => Ok(Json(ClusterOutput::new(cluster))),
    }
}

/// # Get Cluster Users
///
/// Retrieves a list of users who are members of the specified cluster. This operation can be performed by `ROLE_ADMIN` on any cluster within their application.
///
/// ## Roles
///
/// - `ROLE_ADMIN`
///
/// ## Parameters
///
/// - `cluster_id`: A string representing the UUID of the cluster.
///
/// - `page`: An optional integer representing the page number for pagination (default: 1)
///
/// - `page_size`: An optional integer representing the number of items per page for pagination. (default: 10)
///
#[openapi(tag = "Clusters")]
#[get("/clusters/<cluster_id>/users?<page>&<page_size>")]
pub async fn get_cluster_users(
    authorised: Security,
    pool: &rocket::State<DbPool>,
    cluster_id: String,
    page: Option<usize>,
    page_size: Option<usize>,
) -> Result<Json<ListDto<UserOutput>>, CustomError> {
    let pool = pool.inner().to_owned();
    let page = match page {
        None => 1,
        Some(page) => page,
    };
    let page_size = match page_size {
        None => 10,
        Some(page_size) => page_size,
    };
    let cluster_uuid = match Uuid::parse_str(&cluster_id) {
        Err(_) => {
            return Err(ErrorObject::create(
                Status::BadRequest,
                Some("Bad Cluster uuid"),
            ))
        }
        Ok(uuid) => uuid,
    };
    let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
    let connexion_repository: ConnexionRepository = ConnexionContract::new(&pool);
    let cluster_service = ClusterService::new(&pool, application_repository, connexion_repository);

    match authorised.check_roles(Role::ADMIN) {
        false => Err(ErrorObject::create(Status::Unauthorized, None)),
        true => {
            match cluster_service.get_cluster_users(&cluster_uuid, authorised.user, page_size, page)
            {
                Err(status) => Err(ErrorObject::create(status, None)),
                Ok((users, count)) => {
                    let users = users
                        .iter()
                        .map(|user| UserOutput::new(user.clone()))
                        .collect::<Vec<UserOutput>>();
                    Ok(Json(ListDto::new(users, count, page, page_size)))
                }
            }
        }
    }
}
