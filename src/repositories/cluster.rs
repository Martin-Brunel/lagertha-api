use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::dto::cluster::cluster_insertable::ClusterInsertable;
use crate::dto::x_anonymous_sentinel_cluster::x_anonymous_sentinel_cluster_insertable::XAnonymousSentinelClusterInsertable;
use crate::dto::x_sentinel_cluster::x_sentinel_cluster_insertable::XSentinelClusterInsertable;
use crate::dto::x_user_cluster::x_user_cluster_insertable::XUserClusterInsertable;
use crate::models::anonymous_sentinel::AnonymousSentinel;
use crate::models::cluster::Cluster;
use crate::models::sentinel::Sentinel;
use crate::models::user::User;
use crate::models::x_anonymous_sentinel_cluster::XAnonymousSentinelCluster;
use crate::models::x_sentinel_cluster::XSentinelCluster;
use crate::models::x_user_cluster::XUserCluster;
use crate::schema::clusters::dsl::*;
use crate::schema::x_user_cluster::{cluster_id, user_id};
use crate::schema::{anonymous_sentinels, sentinels, users, x_anonymous_sentinel_cluster, x_sentinel_cluster, x_user_cluster};
use crate::{db::connect::DbPool, schema::clusters};

use super::anonymous_sentinel;

pub struct ClusterRepository {
    pool: DbPool,
}

impl ClusterRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn create_cluster(&self, insertable: ClusterInsertable) -> Cluster {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(clusters::table)
            .values(&insertable)
            .returning(Cluster::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn get_user_cluster_by_id(
        &self,
        cluster_to_test_id: &Uuid,
        user_from: &User,
    ) -> Option<Cluster> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match clusters
            .filter(
                id.eq(cluster_to_test_id).and(
                    is_deleted.eq(false).and(
                        created_by_id
                            .eq(user_from.id)
                            .and(application_id.eq(user_from.application.unwrap())),
                    ),
                ),
            )
            .first(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(cluster) => cluster,
        }
    }

    pub fn get_by_id_and_application(
        &self,
        cluster_to_test_id: &Uuid,
        test_application_id: &i32,
    ) -> Option<Cluster> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match clusters
            .find(cluster_to_test_id)
            .filter(is_deleted.eq(false))
            .filter(application_id.eq(test_application_id))
            .first(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(cluster) => cluster,
        }
    }

    /// check if a user is part of a cluster
    pub fn get_user_cluster_memberships(&self, cluster: &Cluster, user_to_test_id: &Uuid) -> bool {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let users = XUserCluster::belonging_to(&cluster)
            .inner_join(users::table.on(x_user_cluster::user_id.eq(users::id)))
            .filter(x_user_cluster::is_deleted.eq(false))
            .filter(users::id.eq(user_to_test_id))
            .filter(users::is_deleted.eq(false))
            .select(users::all_columns)
            .load::<User>(&mut conn)
            .unwrap();
        users.len() > 0
    }

    pub fn add_user_to_cluster(&self, insertable: XUserClusterInsertable) -> XUserCluster {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(x_user_cluster::table)
            .values(&insertable)
            .returning(XUserCluster::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn remove_user_from_cluster(
        &self,
        user: &User,
        cluster: &Cluster,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            x_user_cluster::table
                .filter(user_id.eq(user.id))
                .filter(cluster_id.eq(cluster.id))
                .filter(x_user_cluster::is_deleted.eq(false)),
        )
        .set((
            x_user_cluster::is_deleted.eq(true),
            x_user_cluster::deleted_at.eq(Some(Utc::now())),
            x_user_cluster::deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }

    pub fn delete(
        &self,
        cluster_uuid: Uuid,
        user_from: User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            clusters::table
                .find(cluster_uuid)
                .filter(is_deleted.eq(false)),
        )
        .set((
            is_deleted.eq(true),
            deleted_at.eq(Some(Utc::now())),
            deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }

    /// check if a user is part of a cluster
    pub fn get_user_cluster_sentinels(
        &self,
        cluster: &Cluster,
        sentinel_to_test_id: &Uuid,
    ) -> bool {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let users = XSentinelCluster::belonging_to(&cluster)
            .inner_join(sentinels::table.on(x_sentinel_cluster::sentinel_id.eq(sentinels::id)))
            .filter(
                x_sentinel_cluster::is_deleted
                    .eq(false)
                    .and(sentinels::id.eq(sentinel_to_test_id))
                    .and(sentinels::is_deleted.eq(false)),
            )
            .select(sentinels::all_columns)
            .load::<Sentinel>(&mut conn)
            .unwrap();
        users.len() > 0
    }

    /// check if a user is part of a cluster
    pub fn get_user_cluster_anonymous_sentinels(
        &self,
        cluster: &Cluster,
        sentinel_to_test_id: &Uuid,
    ) -> bool {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let users = XAnonymousSentinelCluster::belonging_to(&cluster)
            .inner_join(anonymous_sentinels::table.on(x_anonymous_sentinel_cluster::anonymous_sentinel_id.eq(anonymous_sentinels::id)))
            .filter(
                x_anonymous_sentinel_cluster::is_deleted
                    .eq(false)
                    .and(anonymous_sentinels::id.eq(sentinel_to_test_id))
                    .and(anonymous_sentinels::is_deleted.eq(false)),
            )
            .select(anonymous_sentinels::all_columns)
            .load::<AnonymousSentinel>(&mut conn)
            .unwrap();
        users.len() > 0
    }

    pub fn add_sentinel_to_cluster(
        &self,
        insertable: XSentinelClusterInsertable,
    ) -> XSentinelCluster {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(x_sentinel_cluster::table)
            .values(&insertable)
            .returning(XSentinelCluster::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn add_anonymous_sentinel_to_cluster(
        &self,
        insertable: XAnonymousSentinelClusterInsertable,
    ) -> XAnonymousSentinelCluster {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(x_anonymous_sentinel_cluster::table)
            .values(&insertable)
            .returning(XAnonymousSentinelCluster::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn remove_anonymous_sentinel_from_cluster(
        &self,
        anonymous_sentinel: &AnonymousSentinel,
        cluster: &Cluster,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            x_anonymous_sentinel_cluster::table.filter(
                x_anonymous_sentinel_cluster::anonymous_sentinel_id
                    .eq(anonymous_sentinel.id)
                    .and(x_anonymous_sentinel_cluster::cluster_id.eq(cluster.id))
                    .and(x_anonymous_sentinel_cluster::is_deleted.eq(false)),
            ),
        )
        .set((
            x_anonymous_sentinel_cluster::is_deleted.eq(true),
            x_anonymous_sentinel_cluster::deleted_at.eq(Some(Utc::now())),
            x_anonymous_sentinel_cluster::deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }


    pub fn remove_sentinel_from_cluster(
        &self,
        sentinel: &Sentinel,
        cluster: &Cluster,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            x_sentinel_cluster::table.filter(
                x_sentinel_cluster::sentinel_id
                    .eq(sentinel.id)
                    .and(x_sentinel_cluster::cluster_id.eq(cluster.id))
                    .and(x_sentinel_cluster::is_deleted.eq(false)),
            ),
        )
        .set((
            x_sentinel_cluster::is_deleted.eq(true),
            x_sentinel_cluster::deleted_at.eq(Some(Utc::now())),
            x_sentinel_cluster::deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }

    pub fn get_cluster_users(
        &self,
        cluster_uuid: &Uuid,
        test_application_id: Option<i32>,
        page: usize,
        page_size: usize,
    ) -> Option<(Vec<User>, i64)> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match users::table
            .inner_join(
                x_user_cluster::table.on(users::id
                    .eq(x_user_cluster::user_id)
                    .and(x_user_cluster::is_deleted.eq(false))),
            )
            .filter(
                x_user_cluster::cluster_id.eq(cluster_uuid).and(
                    users::is_deleted
                        .eq(false)
                        .and(users::application.eq(test_application_id)),
                ),
            )
            .select(users::all_columns)
            .limit(page_size as i64)
            .offset((page_size * (page - 1)) as i64)
            .load::<User>(&mut conn)
        {
            Err(_) => None,
            Ok(users) => match users::table
                .inner_join(
                    x_user_cluster::table.on(users::id
                        .eq(x_user_cluster::user_id)
                        .and(x_user_cluster::is_deleted.eq(false))),
                )
                .filter(
                    x_user_cluster::cluster_id.eq(cluster_uuid).and(
                        users::is_deleted
                            .eq(false)
                            .and(users::application.eq(test_application_id)),
                    ),
                )
                .select(diesel::dsl::count_star()) // Utilisation de count_star pour compter les résultats
                .first::<i64>(&mut conn) {
                    Err(_) => None,
                    Ok(count) => Some((users, count)),
                },
        }
    }
}
