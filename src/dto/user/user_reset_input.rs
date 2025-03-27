use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

use crate::utils::validator::password;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct UserResetInput {
    pub login: String,
    pub application_id: i32,
    pub code: String,
    #[serde(deserialize_with = "password")]
    pub new_password: String,
}
