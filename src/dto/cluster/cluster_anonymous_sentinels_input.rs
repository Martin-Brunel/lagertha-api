use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct ClusterAnonymousSentinelsInput {
    pub anonymous_sentinels: Vec<String>
}
