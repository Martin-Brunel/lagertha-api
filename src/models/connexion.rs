use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::connexions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Connexion {
    pub id: i32,
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
    pub deleted_by_id: Option<uuid::Uuid>
}
