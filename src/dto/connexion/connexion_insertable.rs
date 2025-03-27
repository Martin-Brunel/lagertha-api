use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::connexions::{self};

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = connexions)]
pub struct ConnexionInsertable {
    pub user_id: Uuid,
    pub ip: String,
    pub user_agent: String,
    pub fingerprint: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl ConnexionInsertable {
    pub fn new(user_id: Uuid, ip: String, fingerprint: String, user_agent: String) -> Self {
        ConnexionInsertable {
            user_id,
            ip,
            user_agent,
            fingerprint,
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
