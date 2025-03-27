use crate::utils::validator::password;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]

pub struct UserPasswordInput {
    pub user_id: String,
    #[serde(deserialize_with = "password")]
    pub password: String,
}
