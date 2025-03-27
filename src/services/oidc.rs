use rocket::http::Status;

use crate::{
    db::connect::DbPool, models::user::User, repositories::user::UserRepository,
    utils::open_id::OpenId,
};

pub struct OidcService {
    user_repository: UserRepository,
}

impl OidcService {
    pub fn new(pool: &DbPool) -> Self {
        Self {
            user_repository: UserRepository::new(pool),
        }
    }

    pub fn oidc_verify(
        &self,
        open_id_token: &String,
        user_from: &User,
    ) -> Result<User, (Status, Option<&str>)> {
        let open_id = match OpenId::check(open_id_token.clone()) {
            Err((status, msg)) => return Err((status, msg)),
            Ok(open_id) => open_id,
        };
        match self
            .user_repository
            .get_by_id_and_app(&open_id.sub, user_from.application.unwrap())
        {
            None => Err((Status::Forbidden, None)),
            Some(user) => Ok(user),
        }
    }
}
