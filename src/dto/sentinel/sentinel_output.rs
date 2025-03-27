use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::sentinel::Sentinel;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]

pub struct SentinelOutput {
    pub id: String,
    pub key_size: String,
    pub cipher: String,
    pub sum: String,
}

impl SentinelOutput {
    pub fn new(sentinel: Sentinel, cipher: String) -> Self {
        let key_size = match sentinel.key_size {
            256 => format!("AES-256"),
            _ => format!("AES-128")
        };
        SentinelOutput {
            id: sentinel.id.to_string(),
            cipher,
            key_size,
            sum: sentinel.sum,
        }
    }
}
