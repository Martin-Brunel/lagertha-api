// use crate::{services::user::UserService, utils::pq_kyber::PQKyber, entities::user::User};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{cluster::Cluster, user::User};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct ClusterOutput {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
}

impl ClusterOutput {
    pub fn new(cluster: Cluster) -> Self {
        Self {
            id: cluster.id.to_string(),
            name: cluster.name,
            description: cluster.description,
            created_at: cluster.created_at.to_string(),
        }
    }
}
