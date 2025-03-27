use chrono::Utc;

use crate::{
    db::connect::DbPool,
    dto::application::{
        application_insertable::ApplicationInsertable,
        application_update_input::ApplicationUpdateInput,
    },
    models::{application::Application, user::User},
    traits::application::{ApplicationContract, ApplicationContractWithoutPool},
};

pub struct ApplicationMocks;

impl ApplicationContractWithoutPool for ApplicationMocks {
    fn new_without_pool() -> Self {
        Self
    }
}

impl ApplicationContract for ApplicationMocks {
    fn create_application(&self, insertable: ApplicationInsertable) -> Application {
        Application {
            id: 1,
            name: insertable.name,
            contact_email: insertable.contact_email,
            is_system: insertable.is_system,
            keys_number: 0,
            users_number: 0,
            created_at: Utc::now(),
            is_deleted: false,
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
        }
    }

    fn get_by_id(&self, application_id: i32) -> Option<Application> {
        match application_id {
            999 => None,
            _ => {
                let app = Application {
                    id: application_id,
                    name: format!("test"),
                    contact_email: format!("test@test.fr"),
                    is_system: false,
                    keys_number: 0,
                    users_number: 0,
                    created_at: Utc::now(),
                    is_deleted: false,
                    updated_at: None,
                    deleted_at: None,
                    created_by_id: None,
                    updated_by_id: None,
                    deleted_by_id: None,
                };
                Some(app)
            }
        }
    }

    fn increment_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }

    fn decrement_users(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }

    fn increment_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }

    fn decrement_keys(&self, application_id: &i32) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }

    fn delete_application(
        &self,
        app_id: &i32,
        user_from: &User,
    ) -> Result<usize, diesel::result::Error> {
        Ok(1)
    }

    fn update_application(
        &self,
        input: &ApplicationUpdateInput,
        user_from: &User,
    ) -> Result<Application, diesel::result::Error> {
        match input.id {
            999 => Err(diesel::result::Error::NotFound),
            _ => {
                let app = Application {
                    id: input.id,
                    name: match &input.name {
                        None => format!("test"),
                        Some(name) => name.clone(),
                    },
                    contact_email: match &input.contact_email {
                        None => format!("test@test.com"),
                        Some(contact_email) => contact_email.clone(),
                    },
                    is_system: false,
                    keys_number: 0,
                    users_number: 0,
                    created_at: Utc::now(),
                    is_deleted: false,
                    updated_at: Some(Utc::now()),
                    deleted_at: None,
                    created_by_id: None,
                    updated_by_id: Some(user_from.id),
                    deleted_by_id: None,
                };
                Ok(app)
            }
        }
    }

    fn count_applications(&self) -> Option<i64> {
        Some(1)
    }

    fn new(pool: &DbPool) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
