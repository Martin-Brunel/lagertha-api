use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::clusters::{self};

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = clusters)]
pub struct ClusterInsertable {
    pub name: String,
    pub application_id: i32,
    pub description: Option<String>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl ClusterInsertable {
    pub fn new(
        name: String,
        description: Option<String>,
        application_id: i32,
        user_from_id: Uuid,
    ) -> Self {
        ClusterInsertable {
            name,
            description,
            application_id,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: Some(user_from_id),
            updated_by_id: None,
            deleted_by_id: None,
        }
    }
}
