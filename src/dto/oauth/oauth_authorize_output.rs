use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]

pub struct OauthAuthorizeOutput {
    pub authorization_code: String,
}

impl OauthAuthorizeOutput {
    pub fn new(authorization_code: String) -> Self {
        Self  {
            authorization_code
        }
    }

}
