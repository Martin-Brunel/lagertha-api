use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid::{self, Uuid};


#[derive(Debug, PartialEq, Queryable, Selectable, Identifiable, Clone )]
#[diesel(table_name = crate::schema::clusters)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Cluster {
    pub id: Uuid,
    pub application_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>
}