use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid::{self, Uuid};

use crate::utils::crypto::Crypto;


#[derive(Debug, PartialEq, Queryable, Selectable, Clone )]
#[diesel(table_name = crate::schema::anonymous_sentinels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AnonymousSentinel {
    pub id: Uuid,
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
    pub key_size: i32,
}

impl AnonymousSentinel {
    pub fn check(&self, anonymous_sentinel_private_key: String) -> Result<Self, &'static str> {
        let sum = Crypto::key_sum(&anonymous_sentinel_private_key);
        match sum == self.sum {
            false => Err("Not valid sentinel integrity"),
            true => Ok(self.clone())
        }
    }
}