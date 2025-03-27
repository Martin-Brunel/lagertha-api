use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct AnonymousSentinelPublicInput {
    pub application_id: i32,
}
