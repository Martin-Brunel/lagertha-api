use rocket::http::Status;

use crate::{
    dto::application::{
        application_input::ApplicationInput, application_insertable::ApplicationInsertable, application_update_input::ApplicationUpdateInput,
    },
    models::{application::Application, user::User}, traits::application::ApplicationContract,
};

pub struct ApplicationService<T> {
    application_repository: T,
}

impl<T: ApplicationContract> ApplicationService<T> {
    pub fn new(application_repository: T) -> Self {
        Self {
            application_repository,
        }
    }

    /// find application by id
    pub async fn get_application_by_id(&self, application_id: i32) -> Result<Application, Status> {
        match self.application_repository.get_by_id(application_id) {
            None => Err(Status::NotFound),
            Some(app) => Ok(app),
        }
    }

    pub fn create_application(&self, input: ApplicationInput) -> Application {
        let insertable = ApplicationInsertable::from_input(input, false);
        self.application_repository.create_application(insertable)
    }

    pub fn delete_application(&self, application_id: i32, user_from: User) -> Result<(), Status> {

        let _ = self.application_repository.delete_application(&application_id, &user_from);
        Ok(())
    }

    pub fn update_application(&self, input: ApplicationUpdateInput, user_from: User) -> Result<Application, Status> {

        match self.application_repository.update_application(&input, &user_from) {
            Err(_) => Err(Status::BadRequest),
            Ok(app) => Ok(app)
        }
    }
}
