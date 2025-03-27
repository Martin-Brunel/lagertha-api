use diesel::prelude::*;

use crate::dto::connexion::connexion_insertable::ConnexionInsertable;
use crate::dto::revoked_token::revoked_token_insertable::RevokedTokenInsertable;
use crate::models::revoked_token::RevokedToken;
use crate::models::user::User;
use crate::schema::connexions::user_id;
use crate::schema::revoked_tokens::dsl::*;
use crate::traits::revoked_token::RevokedTokenContract;
use crate::{db::connect::DbPool, schema::revoked_tokens};

pub struct RevokedTokenRepository {
    pool: DbPool,
}

impl RevokedTokenContract for RevokedTokenRepository {
    fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    fn create_revoked_token(&self, insertable: RevokedTokenInsertable) -> RevokedToken {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(revoked_tokens::table)
            .values(&insertable)
            .returning(RevokedToken::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    fn get_by_token(&self, test_token: &String) -> Option<RevokedToken> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match revoked_tokens
            .filter(token.eq(test_token).and(is_deleted.eq(false)))
            .first::<RevokedToken>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(app) => app,
        }
    }
}
