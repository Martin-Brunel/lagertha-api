use rocket::{
    request::{self, Outcome},
    Request,
};
use rocket_okapi::{gen::OpenApiGenerator, okapi::schemars, OpenApiFromRequest};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::errors::ErrorObject;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UserAgent {
    pub user_agent: Option<String>,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for UserAgent {
    type Error = ErrorObject;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_agent = req.headers().get_one("User-Agent").map(|ua| ua.to_string());

        Outcome::Success(UserAgent { user_agent })
    }
}
