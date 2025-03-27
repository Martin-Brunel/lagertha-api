use crate::schema::users::dsl::*;
use crate::utils::password::PasswordUtils;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

use crate::{
    db::connect::DbPool, dto::user::user_insertable::UserInsertable, models::user::User,
    schema::users,
};

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub fn create_user(&self, insertable: UserInsertable) -> User {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(users::table)
            .values(&insertable)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    pub fn count_users(&self) -> Option<i64> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match users
            .filter(is_deleted.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
        {
            Err(_) => None,
            Ok(count) => Some(count),
        }
    }

    pub fn get_by_login_and_app(&self, login_v: &str, app_id: i32) -> Option<User> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match users
            .filter(
                login
                    .eq(login_v)
                    .and(is_deleted.eq(false))
                    .and(application.eq(app_id)),
            )
            .first(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(user) => user,
        }
    }

    pub fn get_by_id_and_app(&self, test_id: &Uuid, app_id: i32) -> Option<User> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match users
            .find(test_id)
            .filter(is_deleted.eq(false))
            .filter(application.eq(app_id))
            .first(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(user) => user,
        }
    }

    pub fn get_by_id(&self, user_id: &Uuid) -> Option<User> {
        let mut conn = match self.pool.get() {
            Err(e) => {
                eprintln!("Failed to get a DB connection: {}", e);
                return None;
            }
            Ok(conn) => conn,
        };
        match users
            .find(user_id)
            .filter(is_deleted.eq(false))
            .first(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(user) => user,
        }
    }

    pub fn update_pq(
        &self,
        user_id: Uuid,
        new_pk: String,
        sk: String,
        other_iv: String,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((
                kyber_public_key.eq(new_pk),
                kyber_secret_key.eq(sk),
                iv.eq(other_iv),
                updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
    }

    pub fn update_validation_code(
        &self,
        new_code: &String,
        user_id: &Uuid,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((
                validation_code.eq(PasswordUtils::hash_password(new_code.to_owned())),
                validation_tries.eq(0),
                updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
    }

    pub fn update_forget_code(
        &self,
        new_code: &String,
        user_id: &Uuid,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let now_plus_15m = Utc::now().checked_add_signed(Duration::minutes(15));

        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((
                validation_code.eq(PasswordUtils::hash_password(new_code.to_owned())),
                validation_tries.eq(0),
                forget_code_delay.eq(now_plus_15m),
                updated_at.eq(Some(Utc::now())),
            ))
            .execute(&mut conn)
    }

    pub fn update_validation_tries(
        &self,
        tries: &i32,
        user_id: &Uuid,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((validation_tries.eq(tries), updated_at.eq(Some(Utc::now()))))
            .execute(&mut conn)
    }

    pub fn validate_account(&self, user_id: &Uuid) -> Result<User, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let _ = diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((is_validated.eq(true), updated_at.eq(Some(Utc::now()))))
            .execute(&mut conn);
        let user = self.get_by_id(user_id).unwrap();
        Ok(user)
    }

    pub fn update_user_password(
        &self,
        user_id: &Uuid,
        new_password: String,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((password.eq(new_password), updated_at.eq(Some(Utc::now()))))
            .execute(&mut conn)
    }

    pub fn delete_user(
        &self,
        user_id: &Uuid,
        user_from_id: &Uuid,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((
                is_deleted.eq(true),
                deleted_at.eq(Some(Utc::now())),
                deleted_by_id.eq(Some(user_from_id)),
            ))
            .execute(&mut conn)
    }

    pub fn activate_2fa(
        &self,
        user_id: &Uuid,
        status: bool,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((is_2fa_activated.eq(status), updated_at.eq(Some(Utc::now()))))
            .execute(&mut conn)
    }

    pub fn update_totp(&self, user_id: &Uuid, new_totp: String) -> User {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        let _ = diesel::update(users::table.find(user_id).filter(is_deleted.eq(false)))
            .set((twofa_code.eq(new_totp), updated_at.eq(Some(Utc::now()))))
            .execute(&mut conn);
        self.get_by_id(user_id).unwrap()
    }
}
