use std::env;

use chrono::Utc;
use reqwest::Error;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::tokio::spawn;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};

pub struct LogHandler;

#[derive(Serialize, Deserialize)]
struct LogDocument {
    timestamp: String,
    level: String,
    method: String,
    uri: String,
    ip: String,
    status: u16,
    user_agent: String,
}

impl LogHandler {
    async fn send_log_to_opensearch(doc: LogDocument) -> Result<(), Error> {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        let index = format!("{}-log", date);
        let opensearch_url = env::var("OPEN_SEARCH_URL").unwrap(); // Changez cela par l'URL de votre cluster OpenSearch
        let url = format!("{}/{}/_doc", opensearch_url, index);
        let opensearch_user = env::var("OPEN_SEARCH_USER").unwrap();
        let opensearch_password = Some(env::var("OPEN_SEARCH_PASSWORD").unwrap());
        
        spawn(async move {
            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();
            let _ = client
                .post(&url)
                .basic_auth(opensearch_user, opensearch_password)
                .json(&doc)
                .send()
                .await;
        });

        Ok(())
    }
}

#[rocket::async_trait]
impl Fairing for LogHandler {
    fn info(&self) -> Info {
        Info {
            name: "Simple Logging Handler",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let ip = request
            .client_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "-".to_string());
        let timestamp = Utc::now().to_rfc3339();
        let method = request.method();
        let uri = request.uri().to_string();
        let status = response.status().code;
        let user_agent = request.headers().get_one("User-Agent").unwrap_or("-");
        let level = match status {
            s if s < 300 => format!("Ok"),
            s if s < 400 => format!("REDIRECT"),
            _ => format!("ERROR"),
        };
        LogHandler::send_log_to_opensearch(LogDocument {
            timestamp: timestamp.to_string(),
            level,
            method: method.to_string(),
            ip,
            status,
            uri,
            user_agent: user_agent.to_string(),
        })
        .await
        .unwrap();
    }
}
