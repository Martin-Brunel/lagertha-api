use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, PartialEq, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::revoked_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RevokedToken {
    pub id: i32,
    pub token: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>
}
