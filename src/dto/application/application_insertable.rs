use chrono::{Utc, DateTime};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::applications;

use super::application_input::ApplicationInput;

// use super::application_input::ApplicationInput;

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = applications)]
pub struct ApplicationInsertable {
    pub name: String,
    pub contact_email: String,
    pub is_system: bool,
    pub users_number: i32,
    pub keys_number: i32,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl ApplicationInsertable {
    pub fn new(name: String, contact_email: String, is_system: bool) -> Self {
        ApplicationInsertable {
            name,
            contact_email,
            is_system,
            keys_number: 0,
            users_number: 0,
            created_at: Utc::now(),
            is_deleted: false,
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
        }
    }
    
    pub fn from_input(input: ApplicationInput, is_system: bool) -> Self {
        ApplicationInsertable {
            name: input.name,
            contact_email: input.contact_email,
            is_system,
            users_number: 0,
            keys_number: 0,
            created_at: Utc::now(),
            is_deleted: false,
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
        }
    }
}
