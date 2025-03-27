use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::anonymous_sentinel::AnonymousSentinel;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]

pub struct AnonymousSentinelOutput {
    pub id: String,
    pub secret_key: String,
    pub sum: String,
    pub key_size: String,
}

impl AnonymousSentinelOutput {
    pub fn new(sentinel: AnonymousSentinel, secret_key: String) -> Self {
        let key_size = match sentinel.key_size {
            1024 => format!("KYBER-1024"),
            768 =>format!("KYBER-768"),
            _ => format!("KYBER-512")
        };
        AnonymousSentinelOutput {
            id: sentinel.id.to_string(),
            secret_key,
            sum: sentinel.sum,
            key_size
        }
    }
}
