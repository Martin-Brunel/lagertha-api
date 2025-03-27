use std::env;

use chrono::Utc;
use rocket::tokio::spawn;

use crate::{
    dto::sentinel_log::sentinel_log_insertable::SentinelLogInsertable,
    models::{sentinel::Sentinel, user::User},
};

pub struct SentinelLogService;

impl SentinelLogService {
    pub async fn new_sentinel_log(sentinel_id: String, user_from: &User, result: bool, ip: &str) {
        let status = match result {
            false => "Error",
            true => "Ok",
        };
        let insertable = SentinelLogInsertable::new(user_from, sentinel_id.clone(), ip, status);
        let index = format!("sentinel-{}-log", user_from.application.unwrap());
        let opensearch_url = env::var("OPEN_SEARCH_URL").unwrap();
        let url = format!("{}/{}/_doc", opensearch_url, index);
        let opensearch_user = env::var("OPEN_SEARCH_USER").unwrap();
        let opensearch_password = Some(env::var("OPEN_SEARCH_PASSWORD").unwrap());
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let _ = client
            .post(&url)
            .basic_auth(opensearch_user, opensearch_password)
            .json(&insertable)
            .send()
            .await;
    }
}
