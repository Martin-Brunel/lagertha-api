use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct AuthRefreshInput {
    pub refresh_token: String,
    pub application_id: i32,
    pub fingerprint: String,
}