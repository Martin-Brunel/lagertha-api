use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;


#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct OauthTokenInput {
    pub authorization_code: String,
    pub state: String,
    pub fingerprint: String,
    pub code_2fa: Option<String>
}
