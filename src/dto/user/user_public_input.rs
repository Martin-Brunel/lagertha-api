use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use crate::utils::validator::{email, password};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct UserPublicInput {
    #[serde(deserialize_with = "email")]
    pub email: String,
    #[serde(deserialize_with = "password")]
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    pub application_id: i32,
}
