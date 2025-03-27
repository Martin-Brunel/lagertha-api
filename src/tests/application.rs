#[cfg(test)]
mod application_tests {
    use std::fmt::format;

    use crate::{
        dto::application::{
            application_input::ApplicationInput, application_update_input::ApplicationUpdateInput,
        },
        mocks::application::ApplicationMocks,
        models::user::User,
        services::application::ApplicationService,
        traits::application::ApplicationContractWithoutPool,
    };

    use super::*;
    use chrono::Utc;
    use rocket::http::Status;
    use rocket::tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn get_application_by_id_success() {
        let application_repository: ApplicationMocks =
            ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let app = application_service
            .get_application_by_id(123)
            .await
            .unwrap();

        assert_eq!(app.id, 123);
    }

    #[tokio::test]
    async fn get_application_by_id_not_found() {
        let application_repository: ApplicationMocks =
            ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let result = application_service.get_application_by_id(999).await;

        assert_eq!(result, Err(Status::NotFound));
    }

    #[tokio::test]
    async fn create_application_success() {
        let application_repository: ApplicationMocks =
            ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let input = ApplicationInput {
            name: String::from("Test App"),
            contact_email: String::from("test@example.com"),
        };

        let app = application_service.create_application(input);

        assert_eq!(app.name, "Test App");
        assert_eq!(app.contact_email, "test@example.com");
        assert_eq!(app.is_system, false);
    }

    #[tokio::test]
    async fn delete_application_success() {
        let application_repository: ApplicationMocks =
            ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let user = User {
            id: Uuid::new_v4(),
            email: format!("test@test.com"),
            firstname: format!("test"),
            lastname: format!("test"),
            twofa_code: format!("123"),
            is_2fa_activated: false,
            login: format!("test"),
            roles: vec![Some(format!("ROLE_USER"))],
            password: Some(format!("test")),
            full_text_search: format!("test"),
            kyber_secret_key: format!("test"),
            kyber_public_key: format!("test"),
            iv: format!("test"),
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
            application: Some(123),
            restricted_ip: vec![],
            is_validated: true,
            validation_code: None,
            validation_tries: 0,
            forget_code_delay: None,
        };
        let result = application_service.delete_application(123, user);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_application_success() {
        let application_repository: ApplicationMocks =
            ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let user = User {
            id: Uuid::new_v4(),
            email: format!("test@test.com"),
            firstname: format!("test"),
            lastname: format!("test"),
            twofa_code: format!("123"),
            is_2fa_activated: false,
            login: format!("test"),
            roles: vec![Some(format!("ROLE_USER"))],
            password: Some(format!("test")),
            full_text_search: format!("test"),
            kyber_secret_key: format!("test"),
            kyber_public_key: format!("test"),
            iv: format!("test"),
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
            application: Some(123),
            restricted_ip: vec![],
            is_validated: true,
            validation_code: None,
            validation_tries: 0,
            forget_code_delay: None,
        };
        let input = ApplicationUpdateInput {
            id: 123,
            name: Some(String::from("Updated App")),
            contact_email: Some(String::from("updated@example.com")),
        };

        let app = application_service.update_application(input, user).unwrap();

        assert_eq!(app.name, "Updated App");
        assert_eq!(app.contact_email, "updated@example.com");
    }

    #[tokio::test]
    async fn update_application_failure() {
        let application_repository: ApplicationMocks = ApplicationContractWithoutPool::new_without_pool();
        let application_service = ApplicationService::new(application_repository);

        let user = User {
            id: Uuid::new_v4(),
            email: format!("test@test.com"),
            firstname: format!("test"),
            lastname: format!("test"),
            twofa_code: format!("123"),
            is_2fa_activated: false,
            login: format!("test"),
            roles: vec![Some(format!("ROLE_USER"))],
            password: Some(format!("test")),
            full_text_search: format!("test"),
            kyber_secret_key: format!("test"),
            kyber_public_key: format!("test"),
            iv: format!("test"),
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
            application: Some(123),
            restricted_ip: vec![],
            is_validated: true,
            validation_code: None,
            validation_tries: 0,
            forget_code_delay: None,
        };
        let input = ApplicationUpdateInput {
            id: 999,  // ID non existant
            name: Some(String::from("Updated App")),
            contact_email: Some(String::from("updated@example.com")),
        };

        let result = application_service.update_application(input, user);

        assert_eq!(result, Err(Status::BadRequest));
    }
}
