use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use otpauth::TOTP;

use crate::models::{application::Application, user::User};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct UserTotpCode {
    pub totp_url: String,
}


impl UserTotpCode {
    pub fn new(user: User, application: Application) -> Self {
        let url = format!("otpauth://totp/{}?secret={}&issuer={}&algorithm=SHA1&digits=6&period=30", user.login, user.twofa_code, application.name);
        UserTotpCode {
            totp_url: url
        }
    }
}