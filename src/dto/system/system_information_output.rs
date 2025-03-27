use std::{env, fs, path::Path};

use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::LICENSE_VALID;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub struct SystemInformationOutput {
    pub nb_users: i64,
    pub nb_applications: i64,
    pub nb_sentinels: i64,
    pub nb_anonymous_sentinels: i64,
}

impl SystemInformationOutput {
    pub fn new(
        nb_users: i64,
        nb_applications: i64,
        nb_sentinels: i64,
        nb_anonymous_sentinels: i64,
    ) -> Self {
        SystemInformationOutput {
            nb_users,
            nb_applications,
            nb_sentinels,
            nb_anonymous_sentinels,
        }
    }
}
