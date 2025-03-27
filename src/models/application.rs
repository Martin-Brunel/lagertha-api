use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid;


#[derive(Debug, PartialEq, Queryable, Selectable )]
#[diesel(table_name = crate::schema::applications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Application {
    pub id: i32,
    pub name: String,
    pub contact_email: String,
    pub is_system: bool,
    pub keys_number: i32,
    pub users_number: i32,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>
}