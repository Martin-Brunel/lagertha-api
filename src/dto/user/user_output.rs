use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

use crate::models::user::User;

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]

pub struct UserOutput {
    pub id: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub login: String,
    pub roles: Vec<Option<String>>,
    pub created_at: String,
}

impl UserOutput {
    pub fn new(user: User) -> Self {
        UserOutput {
            id: user.id.to_string(),
            email: user.email,
            login: user.login,
            firstname: user.firstname,
            lastname: user.lastname,
            roles: user.roles,
            created_at: user.created_at.to_string(),
        }
    }


}
