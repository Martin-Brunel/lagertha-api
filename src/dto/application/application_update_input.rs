use crate::utils::validator::option_email;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct ApplicationUpdateInput {
    pub id: i32,
    pub name: Option<String>,
    #[serde(deserialize_with = "option_email")]
    pub contact_email: Option<String>,
}
