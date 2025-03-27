use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]

pub struct User2faActivateInput {
    pub is_activate: bool,
    pub code: String,
}