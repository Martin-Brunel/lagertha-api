use std::{env, fs, path::Path};

use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::LICENSE_VALID;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct SystemVersionOutput {
    pub version: String,
    pub name: String,
    pub api_mode: String,
    pub license_number: Option<String>,
    pub license_expiration: Option<String>,
    pub license_name: Option<String>,

}

impl SystemVersionOutput {
    pub fn new() -> Self {
        let license_valid = LICENSE_VALID.lock().unwrap();
        let license_number = match license_valid.clone() {
            None => None,
            Some(license) => Some(license.license_key)
        };
        let license_expiration = match license_valid.clone() {
            None => None,
            Some(license) => Some(license.expiration_date.to_string())
        };

        let api_mode = match license_valid.clone() {
            None => format!("Community"),
            Some(license) => license.mode
        };

        let license_name = match license_valid.clone() {
            None => None,
            Some(license) => Some(license.user_name.to_string())
        };
        
        SystemVersionOutput {
            version: env!("CARGO_PKG_VERSION").to_string(),
            name: env!("CARGO_PKG_NAME").to_string(),
            api_mode,
            license_number,
            license_expiration,
            license_name
        }
    }
}
