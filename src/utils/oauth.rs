use std::env;

use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mail_send::smtp::auth;
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{controlers::oauth, models::user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Oauth {
    pub id: Uuid,
    pub state_control: String,
    pub iat: i64,
    pub exp: i64,
}

impl Oauth {
    fn new(user_id: Uuid, state: String) -> Self {
        let now = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(state);
        let state_control = format!("{:x}", hasher.finalize());
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                env::var("OAUTH_TOKEN_DURATION")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            ))
            .expect("invalid timestamp")
            .timestamp();
        Self {
            id: user_id,
            state_control,
            iat: now,
            exp: expiration
        }
    }

    pub fn create(user: User, state: String) -> String {
        let data = Self::new(user.id, state);
        let key = EncodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/private.key")).unwrap();
        encode(&Header::new(Algorithm::RS256), &data, &key).unwrap()
    }

    pub async fn check(authorize_code: String, state: String) -> Result<Oauth, (Status, Option<&'static str>)> {
        match decode::<Self>(
            &authorize_code,
            &DecodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/public.key")).unwrap(),
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(authorized) => {
                let oauth = authorized.claims;
                let mut hasher = Sha256::new();
                hasher.update(state);
                let state_control = format!("{:x}", hasher.finalize());
                match state_control == oauth.state_control {
                    false => Err((Status::Forbidden, None)),
                    true => Ok(oauth)
                }
            },
            Err(_) => Err((Status::Forbidden, None))
        }
    }
}
