use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct UserForgetInput {
    pub login: String,
    pub application_id: i32
}
