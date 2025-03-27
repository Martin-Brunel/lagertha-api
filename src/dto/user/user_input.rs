use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use crate::utils::validator::{email, option_password};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct UserInput {
    #[serde(deserialize_with = "email")]
    pub email: String,
    #[serde(deserialize_with = "option_password")]
    pub password: Option<String>,
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    pub application_id: i32,
    pub is_admin: bool,
}
