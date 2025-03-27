use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::{sentinel::Sentinel, user::User};

#[derive(Deserialize, Serialize)]
pub struct SentinelLogInsertable {
    timestamp: String,
    sentinel_id: String,
    user_from_id: String,
    result: String,
    ip: String,
    application_id: i32,
}

impl SentinelLogInsertable {
    pub fn new(user_from: &User, sentinel_id: String, ip: &str, result: &str) -> Self {
        Self {
            timestamp: Utc::now().to_rfc3339(),
            sentinel_id: sentinel_id,
            user_from_id: user_from.id.to_string(),
            result: result.to_string(),
            ip: ip.to_string(),
            application_id: user_from.application.unwrap(),
        }
    }
}