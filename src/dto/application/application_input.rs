use crate::utils::validator::email;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct ApplicationInput {
    pub name: String,
    #[serde(deserialize_with = "email")]
    pub contact_email: String,
}
