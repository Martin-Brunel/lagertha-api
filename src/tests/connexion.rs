#[cfg(test)]
mod connexion_tests {
    use crate::{
        mocks::connexion::ConnexionMocks,
        services::connexion::ConnexionService,
        traits::connexion::ConnexionContractWithoutPool,
        models::user::User,
    };

    use super::*;
    use chrono::Utc;
    use rocket::tokio;
    use uuid::Uuid;

    #[tokio::test]
    async fn create_connexion_success() {
        let connexion_repository: ConnexionMocks = ConnexionContractWithoutPool::new_without_pool();
        let connexion_service = ConnexionService::new(connexion_repository);

        let ip = String::from("192.168.1.1");
        let user_agent = String::from("user/agent test");
        let fingerprint = String::from("fingerprint test");
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

        let connexion = connexion_service.create_connexion(&ip, &user_agent, &fingerprint, &user);

        assert_eq!(connexion.user_id, user.id);
        assert_eq!(connexion.ip, ip);
        assert_eq!(connexion.user_agent, user_agent);
        assert_eq!(connexion.fingerprint, fingerprint);
    }
}