use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{models::anonymous_sentinel::AnonymousSentinel, utils::pq_kyber::PQKyber};

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct AnonymousSentinelPublicOutput {
    pub id: String,
    pub public_key: String,
    pub sum: String,
    pub key_size: String,

}

impl AnonymousSentinelPublicOutput {
    pub fn new(sentinel: AnonymousSentinel) -> Self {
        let key_size = match sentinel.key_size {
            1024 => format!("KYBER-1024"),
            768 =>format!("KYBER-768"),
            _ => format!("KYBER-512")
        };
        AnonymousSentinelPublicOutput {
            id: sentinel.id.to_string(),
            public_key: PQKyber::decrypt_key(sentinel.public_key, sentinel.iv),
            sum: sentinel.sum,
            key_size
        }
    }
}
