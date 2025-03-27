use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};

use crate::schema::revoked_tokens;

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = revoked_tokens)]
pub struct RevokedTokenInsertable {
    pub token: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl RevokedTokenInsertable {
    pub fn new(token: String) -> Self {
        RevokedTokenInsertable {
            token,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
        }
    }
}
