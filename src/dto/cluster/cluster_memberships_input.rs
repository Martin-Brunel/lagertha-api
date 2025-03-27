use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct ClusterMembershipsInput {
    pub memberships: Vec<String>
}
