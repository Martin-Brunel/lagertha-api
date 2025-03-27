use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::x_user_cluster;

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = x_user_cluster)]
pub struct XUserClusterInsertable {
    pub user_id: Uuid,
    pub cluster_id: Uuid,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
}

impl XUserClusterInsertable {
    pub fn new(
        cluster_id: Uuid,
        user_id: Uuid,
        user_from_id: Uuid
    ) -> Self {
        XUserClusterInsertable {
            user_id,
            cluster_id,
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
