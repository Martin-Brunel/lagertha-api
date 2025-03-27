use crate::db::connect::DbPool;
use crate::dto::anonymous_sentinel::anonymous_sentinel_insertable::AnonymousSentinelInsertable;
use crate::models::anonymous_sentinel::AnonymousSentinel;
use crate::models::user::User;
use crate::schema::anonymous_sentinels::{application_id, created_by_id, deleted_at, deleted_by_id, is_deleted};
use crate::schema::{anonymous_sentinels, clusters, users, x_anonymous_sentinel_cluster, x_user_cluster};
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

pub struct AnonymousSentinelRepository {
    pool: DbPool,
}

impl AnonymousSentinelRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn create_anonymous_sentinel(&self, insertable: AnonymousSentinelInsertable) -> AnonymousSentinel {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(anonymous_sentinels::table)
            .values(&insertable)
            .returning(AnonymousSentinel::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn get_anonymous_sentinel_by_id(&self, sentinel_uuid: &Uuid, user_from: &User) -> Option<AnonymousSentinel> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match user_from.clone().roles.into_iter().find(|role| {
            if role.is_some() {
                let role = role.clone().unwrap();
                return role == format!("ROLE_ADMIN");
            }
            false
        }) {
            None => {
                match anonymous_sentinels::table
                    .left_join(
                        x_anonymous_sentinel_cluster::table.on(x_anonymous_sentinel_cluster::anonymous_sentinel_id
                            .eq(anonymous_sentinels::id)
                            .and(x_anonymous_sentinel_cluster::is_deleted.eq(false))),
                    )
                    .left_join(
                        clusters::table.on(clusters::id
                            .eq(x_anonymous_sentinel_cluster::cluster_id)
                            .and(clusters::is_deleted.eq(false))),
                    )
                    .left_join(
                        x_user_cluster::table.on(x_user_cluster::cluster_id
                            .eq(clusters::id)
                            .and(x_user_cluster::is_deleted.eq(false))),
                    )
                    .left_join(
                        users::table.on(users::id
                            .eq(x_user_cluster::user_id)
                            .and(users::is_deleted.eq(false))),
                    )
                    .filter(
                        anonymous_sentinels::id
                            .eq(sentinel_uuid)
                            .and(anonymous_sentinels::is_deleted.eq(false)),
                    )
                    .filter(
                        users::id
                            .eq(user_from.id)
                            .or(anonymous_sentinels::created_by_id.eq(user_from.id)),
                    )
                    .select(anonymous_sentinels::all_columns)
                    .first::<AnonymousSentinel>(&mut conn)
                    .optional()
                {
                    Err(_) => None,
                    Ok(res) => {
        
                        res
                    },
                }
            },
            Some(user) => {
                match anonymous_sentinels::table
                .filter(
                    anonymous_sentinels::id
                        .eq(sentinel_uuid)
                        .and(anonymous_sentinels::is_deleted.eq(false))
                        .and(anonymous_sentinels::application_id.eq(&user_from.application.unwrap())),
                )
                .select(anonymous_sentinels::all_columns)
                .first::<AnonymousSentinel>(&mut conn)
                .optional()
            {
                Err(_) => None,
                Ok(res) => {
    
                    res
                },
            }
            }
        }
        

    }

    pub fn get_public_anonymous_sentinel_by_id(&self, sentinel_uuid: &Uuid) -> Option<AnonymousSentinel> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match anonymous_sentinels::table
            .filter(
                anonymous_sentinels::id
                    .eq(sentinel_uuid)
                    .and(anonymous_sentinels::is_deleted.eq(false)),
            )
            .select(anonymous_sentinels::all_columns)
            .first::<AnonymousSentinel>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(res) => {

                res
            },
        }

    }

    pub fn delete_sentinel_by_id_admin(
        &self,
        sentinel_uuid: &Uuid,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            anonymous_sentinels::table.filter(
                anonymous_sentinels::id
                    .eq(sentinel_uuid)
                    .and(application_id.eq(user_from.application.unwrap()))
                    .and(is_deleted.eq(false)),
            ),
        )
        .set((
            is_deleted.eq(true),
            deleted_at.eq(Some(Utc::now())),
            deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }

    pub fn delete_sentinel_by_id_user(
        &self,
        sentinel_uuid: &Uuid,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            anonymous_sentinels::table.filter(
                anonymous_sentinels::id
                    .eq(sentinel_uuid)
                    .and(application_id.eq(user_from.application.unwrap()))
                    .and(is_deleted.eq(false))
                    .and(created_by_id.eq(user_from.id)),
            ),
        )
        .set((
            is_deleted.eq(true),
            deleted_at.eq(Some(Utc::now())),
            deleted_by_id.eq(user_from.id),
        ))
        .execute(&mut conn)
    }

    pub fn count_anonymous_sentinels(&self) -> Option<i64> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match anonymous_sentinels::table
            .filter(is_deleted.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
        {
            Err(_) => None,
            Ok(count) => Some(count),
        }
    }

}
