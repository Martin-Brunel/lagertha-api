use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::cluster::Cluster;
use crate::models::user::User;
use crate::schema::x_user_cluster;

#[derive(Identifiable, Debug, Queryable, Selectable, Associations, PartialEq)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Cluster, foreign_key = cluster_id))]
#[diesel(table_name = x_user_cluster)]
pub struct XUserCluster {
    pub id: i32,
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
