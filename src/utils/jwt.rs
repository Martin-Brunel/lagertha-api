use crate::models::{application::Application, user::User};
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Jwt {
    pub id: Uuid,
    pub login: String,
    pub application_id: i32,
    pub application_name: String,
    pub is_refresh: bool,
    pub is_2fa_activate: bool,
    pub iat: i64,
    pub exp: i64,
    pub roles: Vec<Option<String>>,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub device_id: Option<String>,
}

impl Jwt {
    pub async fn new(
        data: &User,
        application: &Application,
        is_refresh: bool,
        device_sha: Option<String>,
    ) -> Self {
        dotenv().ok();
        let now = Utc::now().timestamp();
        let expiration = match is_refresh {
            true => Utc::now()
                .checked_add_signed(chrono::Duration::days(
                    env::var("JWT_REFRESH_TOKEN_DURATION")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ))
                .expect("valid timestamp")
                .timestamp(),
            _ => Utc::now()
                .checked_add_signed(chrono::Duration::seconds(
                    env::var("JWT_TOKEN_DURATION")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ))
                .expect("valid timestamp")
                .timestamp(),
        };

        Jwt {
            id: data.id,
            login: data.login.to_string(),
            application_id: application.id,
            application_name: application.name.to_string(),
            is_2fa_activate: data.is_2fa_activated,
            iat: now,
            is_refresh,
            exp: expiration,
            roles: data.roles.clone(),
            firstname: data.firstname.to_string(),
            lastname: data.lastname.to_string(),
            email: data.email.to_string(),
            device_id: device_sha,
        }
    }

    pub async fn create_jwt(
        data: &User,
        is_refresh: bool,
        application: &Application,
        device_sha: Option<String>,
    ) -> String {
        let data = Jwt::new(data, application, is_refresh, device_sha).await;
        let key = EncodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/private.key")).unwrap();
        encode(&Header::new(Algorithm::RS256), &data, &key).unwrap()
    }
}
