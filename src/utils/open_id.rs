use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::connect::DbPool,
    models::{application::Application, user::User},
    repositories::{application::ApplicationRepository, connexion::ConnexionRepository},
};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenId {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
}

impl OpenId {
    fn new(data: User) -> Self {
        dotenv().ok();
        let now = Utc::now().timestamp();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                env::var("OPENID_TOKEN_DURATION")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            ))
            .expect("invalid timestamp")
            .timestamp();

        OpenId {
            sub: data.id,
            iat: now,
            exp: expiration,
        }
    }

    pub async fn create(data: User) -> String {
        let data = Self::new(data);
        let key = EncodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/private.key")).unwrap();
        encode(&Header::new(Algorithm::RS256), &data, &key).unwrap()
    }

    pub fn check(open_id_token: String) -> Result<Self, (Status, Option<&'static str>)> {
        match decode::<Self>(
            &open_id_token,
            &DecodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/public.key")).unwrap(),
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(open_id) => Ok(open_id.claims),
            Err(_) => Err((Status::Forbidden, None))
        }
    }
}
