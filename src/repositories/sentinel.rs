use std::time::Instant;

use crate::db::connect::DbPool;
use crate::dto::sentinel::sentinel_insertable::SentinelInsertable;
use crate::models::sentinel::Sentinel;
use crate::models::user::User;
use crate::schema::x_sentinel_cluster::sentinel_id;
use crate::schema::{
    clusters,
    sentinels::{self, *},
    users, x_sentinel_cluster, x_user_cluster,
};
use chrono::Utc;
use diesel::prelude::*;
use rocket_okapi::okapi::schemars::schema;
use uuid::Uuid;

pub struct SentinelRepository {
    pool: DbPool,
}

impl SentinelRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn create_sentinel(&self, insertable: SentinelInsertable) -> Sentinel {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(sentinels::table)
            .values(&insertable)
            .returning(Sentinel::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn get_sentinel_by_id(&self, sentinel_uuid: &Uuid, user_from: &User) -> Option<Sentinel> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match user_from.clone().roles.into_iter().find(|role| {
            if role.is_some() {
                let role = role.clone().unwrap();
                return role == format!("ROLE_ADMIN");
            }
            false
        }) {
                None => {
                    match sentinels::table
                    .left_join(
                        x_sentinel_cluster::table.on(x_sentinel_cluster::sentinel_id
                            .eq(sentinels::id)
                            .and(x_sentinel_cluster::is_deleted.eq(false))),
                    )
                    .left_join(
                        clusters::table.on(clusters::id
                            .eq(x_sentinel_cluster::cluster_id)
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
                        sentinels::id
                            .eq(sentinel_uuid)
                            .and(sentinels::is_deleted.eq(false))
                            .and(sentinels::application_id.eq(&user_from.application.unwrap())),
                    )
                    .filter(
                        users::id
                            .eq(user_from.id)
                            .or(sentinels::created_by_id.eq(user_from.id)),
                    )
                    .select(sentinels::all_columns)
                    .first::<Sentinel>(&mut conn)
                    .optional()
                {
                    Err(_) => None,
                    Ok(res) => {
        
                        res
                    },
                }
                },
                Some(user) => {
                    match sentinels::table
                    .filter(
                        sentinels::id
                            .eq(sentinel_uuid)
                            .and(sentinels::is_deleted.eq(false))
                            .and(sentinels::application_id.eq(&user_from.application.unwrap())),
                    )
                    .select(sentinels::all_columns)
                    .first::<Sentinel>(&mut conn)
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

    pub fn count_sentinels(&self) -> Option<i64> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match sentinels::table
            .filter(is_deleted.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
        {
            Err(_) => None,
            Ok(count) => Some(count),
        }
    }


    

    pub fn delete_sentinel_by_id_admin(
        &self,
        sentinel_uuid: &Uuid,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            sentinels::table.filter(
                sentinels::id
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
            sentinels::table.filter(
                sentinels::id
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
}
