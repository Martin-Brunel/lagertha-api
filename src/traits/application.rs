use crate::{
    db::connect::DbPool,
    dto::application::{
        application_insertable::ApplicationInsertable,
        application_update_input::ApplicationUpdateInput,
    },
    models::{application::Application, user::User},
};

pub trait ApplicationContract {
    /// create a new instance
    fn new(pool: &DbPool) -> Self
    where
        Self: Sized;

    fn create_application(&self, insertable: ApplicationInsertable) -> Application;

    fn get_by_id(&self, application_id: i32) -> Option<Application>;

    fn increment_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error>;

    fn decrement_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error>;

    fn increment_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error>;

    fn decrement_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error>;

    fn delete_application(
        &self,
        app_id: &i32,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error>;

    fn update_application(
        &self,
        input: &ApplicationUpdateInput,
        user_from: &User,
    ) -> Result<Application, diesel::result::Error>;

    fn count_applications(&self) -> Option<i64>;
}

pub trait ApplicationContractWithoutPool: ApplicationContract {
    fn new_without_pool() -> Self
    where
        Self: Sized;
}
