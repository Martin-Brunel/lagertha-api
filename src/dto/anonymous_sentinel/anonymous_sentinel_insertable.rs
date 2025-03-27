use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::anonymous_sentinels::{self};

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = anonymous_sentinels)]
pub struct AnonymousSentinelInsertable {
    pub application_id: i32,
    pub iv: String,
    pub sum: String,
    pub public_key: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
    pub key_size: i32
}

impl AnonymousSentinelInsertable {
    pub fn new(
        iv: String,
        sum: String,
        public_key: String,
        application_id: i32,
        user_from_id: Option<Uuid>,
        key_size: i32
    ) -> Self {
        AnonymousSentinelInsertable {
            application_id,
            iv,
            sum,
            public_key,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: user_from_id,
            updated_by_id: None,
            deleted_by_id: None,
            key_size
        }
    }
}
