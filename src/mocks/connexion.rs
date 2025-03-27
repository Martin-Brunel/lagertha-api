use chrono::Utc;

use crate::{
    db::connect::DbPool,
    dto::{application::{
        application_insertable::ApplicationInsertable,
        application_update_input::ApplicationUpdateInput,
    }, connexion::connexion_insertable::ConnexionInsertable},
    models::{application::Application, connexion::Connexion, user::User},
    traits::connexion::{ConnexionContract, ConnexionContractWithoutPool},
};

pub struct ConnexionMocks;

impl ConnexionContractWithoutPool for ConnexionMocks {
    fn new_without_pool() -> Self {
        Self
    }
}

impl ConnexionContract for ConnexionMocks {

    fn new(pool: &DbPool) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn create_connexion(&self, insertable: ConnexionInsertable) -> Connexion {
        Connexion {
            id: 123,
            user_id: insertable.user_id,
            ip: insertable.ip,
            user_agent: insertable.user_agent,
            fingerprint: insertable.fingerprint,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
        }
    }

    fn get_user_last_connexion(&self, user: &User) -> Option<Connexion> {
        match user.login.as_str() {
            "no-connect" => None,
            _ => Some(Connexion {
                id: 123,
                user_id: user.id,
                ip: format!("127.0.0.1"),
                user_agent: format!("user/agent test"),
                fingerprint: format!("fingerprint test"),
                is_deleted: false,
                created_at: Utc::now(),
                updated_at: None,
                deleted_at: None,
                created_by_id: None,
                updated_by_id: None,
                deleted_by_id: None,
            })
        }
    }

    fn get_user_ip_connexion(&self, user: &User, test_ip: &String) -> Option<Connexion> {
        match user.login.as_str() {
            "no-connect" => None,
            _ => Some(Connexion {
                id: 123,
                user_id: user.id,
                ip: test_ip.clone(),
                user_agent: format!("user/agent test"),
                fingerprint: format!("fingerprint test"),
                is_deleted: false,
                created_at: Utc::now(),
                updated_at: None,
                deleted_at: None,
                created_by_id: None,
                updated_by_id: None,
                deleted_by_id: None,
            })
        }
    }

    fn get_user_fingerprint_connexion(
        &self,
        user: &User,
        test_fingerprint: &String,
    ) -> Option<Connexion> {
        match user.login.as_str() {
            "no-connect" => None,
            _ => Some(Connexion {
                id: 123,
                user_id: user.id,
                ip: format!("127.0.0.1"),
                user_agent: format!("user/agent test"),
                fingerprint: test_fingerprint.clone(),
                is_deleted: false,
                created_at: Utc::now(),
                updated_at: None,
                deleted_at: None,
                created_by_id: None,
                updated_by_id: None,
                deleted_by_id: None,
            })
        }
    }
}
