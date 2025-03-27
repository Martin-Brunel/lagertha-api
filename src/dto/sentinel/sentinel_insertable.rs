use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::sentinels::{self};

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = sentinels)]
pub struct SentinelInsertable {
    pub application_id: i32,
    pub iv: String,
    pub sum: String,
    pub key_size: i32,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl SentinelInsertable {
    pub fn new(
        iv: String,
        sum: String,
        application_id: i32,
        user_from_id: Uuid,
        key_size: i32
    ) -> Self {
        SentinelInsertable {
            application_id,
            iv,
            sum,
            key_size,
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
