use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::query_builder::AsChangeset;
use diesel::result::Error;
use uuid::Uuid;

use crate::dto::application::application_update_input::ApplicationUpdateInput;
use crate::models::application::Application;
use crate::models::user::User;
use crate::schema::applications::dsl::*;
use crate::traits::application::ApplicationContract;
use crate::{
    db::connect::DbPool, dto::application::application_insertable::ApplicationInsertable,
    schema::applications,
};

#[derive(AsChangeset)]
#[table_name = "applications"]
struct ApplicationChangeset {
    name: Option<String>,
    contact_email: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    updated_by_id: Option<Uuid>,
}

pub struct ApplicationRepository {
    pool: DbPool,
}

impl ApplicationContract for ApplicationRepository {
    fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    fn create_application(&self, insertable: ApplicationInsertable) -> Application {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(applications::table)
            .values(&insertable)
            .returning(Application::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    fn get_by_id(&self, application_id: i32) -> Option<Application> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match applications
            .find(application_id)
            .filter(is_deleted.eq(false))
            .first::<Application>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(app) => app,
        }
    }

    fn increment_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            applications::table
                .find(application_id)
                .filter(is_deleted.eq(false)),
        )
        .set((
            users_number.eq(users_number + 1),
            updated_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
    }

    fn decrement_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            applications::table
                .find(application_id)
                .filter(is_deleted.eq(false)),
        )
        .set((
            users_number.eq(users_number - 1),
            updated_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
    }

    fn increment_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            applications::table
                .find(application_id)
                .filter(is_deleted.eq(false)),
        )
        .set((
            keys_number.eq(keys_number + 1),
            updated_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
    }

    fn decrement_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            applications::table
                .find(application_id)
                .filter(is_deleted.eq(false)),
        )
        .set((
            keys_number.eq(keys_number - 1),
            updated_at.eq(Some(Utc::now())),
        ))
        .execute(&mut conn)
    }

    fn delete_application(
        &self,
        app_id: &i32,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::update(
            applications::table
                .find(app_id)
                .filter(is_deleted.eq(false)),
        )
        .set((
            is_deleted.eq(true),
            deleted_at.eq(Some(Utc::now())),
            deleted_by_id.eq(Some(user_from.id)),
        ))
        .execute(&mut conn)
    }

    fn update_application(
        &self,
        input: &ApplicationUpdateInput,
        user_from: &User,
    ) -> Result<Application, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");

        let changes = ApplicationChangeset {
            name: input.name.clone(),
            contact_email: input.contact_email.clone(),
            updated_at: Some(chrono::Utc::now()),
            updated_by_id: Some(user_from.id),
        };

        let updated_rows = diesel::update(
            applications::table
                .find(input.id)
                .filter(applications::is_deleted.eq(false)),
        )
        .set(&changes)
        .execute(&mut conn)?;

        if updated_rows == 0 {
            Err(Error::NotFound)
        } else {
            self.get_by_id(input.id).ok_or(Error::NotFound)
        }
    }

    fn count_applications(&self) -> Option<i64> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match applications
            .filter(is_deleted.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
        {
            Err(_) => None,
            Ok(count) => Some(count),
        }
    }
}
