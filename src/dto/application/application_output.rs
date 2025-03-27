use serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;

use crate::models::application::Application;

#[derive(Deserialize, Serialize, Debug,JsonSchema)]
pub struct ApplicationOutput {
    id: i32,
    name: String,
    users_number: i32,
    keys_number: i32,
    is_system: bool,
    contact_email: String,
    created_at: String,
}

impl ApplicationOutput {
    pub fn new(application: Application) -> Self {
        ApplicationOutput {
            id: application.id,
            is_system: application.is_system,
            users_number: application.users_number,
            keys_number: application.keys_number,
            name: application.name,
            contact_email: application.contact_email,
            created_at: application.created_at.to_string()
        }
    }

    pub fn from_vec(applications: Vec<Application>) ->Vec<Self> {
        applications.into_iter().map(|app| {
            ApplicationOutput::new(app)
        }).collect()
    }
}
