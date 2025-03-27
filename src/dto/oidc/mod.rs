use crate::utils::validator::{email, password};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct OidcVerifyInput {
    pub open_id_token: String,
}
