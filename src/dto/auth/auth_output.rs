// use crate::{services::user::UserService, utils::pq_kyber::PQKyber, entities::user::User};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{db::connect::DbPool, models::user::User, services::user::UserService, utils::pq_kyber::PQKyber};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct AuthOutput {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub open_id: String,
}

impl AuthOutput {
    pub async fn new(
        access_token: String,
        refresh_token: String,
        open_id: String,
        // user: &User,
        // pool: &DbPool
    ) -> Self {
        // let pk = {
        //     let user_service = UserService::new(pool);
        //     let u = user_service.reinit_kyber_keypair(user)
        //         .await
        //         .unwrap();
        //     PQKyber::decrypt_key(u.kyber_public_key, u.iv)
        // };
        AuthOutput {
            access_token,
            token_type: String::from("Bearer"),
            refresh_token,
            open_id,
        }
    }
}
