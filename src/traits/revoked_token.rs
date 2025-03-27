use crate::{db::connect::DbPool, dto::revoked_token::revoked_token_insertable::RevokedTokenInsertable, models::revoked_token::RevokedToken};

pub trait RevokedTokenContract {
    /// create a new instance
    fn new(pool: &DbPool) -> Self where Self: Sized;

    fn create_revoked_token(&self, insertable: RevokedTokenInsertable) -> RevokedToken;

    fn get_by_token(&self, test_token: &String) -> Option<RevokedToken>;

}