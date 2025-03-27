use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;


#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct ClusterInput {
    pub name: String,
    pub description: Option<String>,
    pub memberships: Vec<String>
}
