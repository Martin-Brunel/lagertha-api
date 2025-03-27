use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid::{self, Uuid};

use super::sentinel::Sentinel;
use crate::models::cluster::Cluster;


#[derive(Identifiable, Debug, Queryable, Selectable, Associations, PartialEq)]
#[diesel(table_name = crate::schema::x_sentinel_cluster)]
#[diesel(belongs_to(Sentinel, foreign_key = sentinel_id))]
#[diesel(belongs_to(Cluster, foreign_key = cluster_id))]
pub struct XSentinelCluster {
    pub id: i32,
    pub sentinel_id: Uuid,
    pub cluster_id: Uuid,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>
}