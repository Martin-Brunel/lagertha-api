use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;


#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct AuthInput {
    pub login: String,
    pub password: String,
    pub application_id: i32,
    pub fingerprint: String,
    pub code_2fa: Option<String>
}
