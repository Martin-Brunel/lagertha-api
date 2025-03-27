use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct ClusterSentinelsInput {
    pub sentinels: Vec<String>
}
