use crate::{
    db::connect::DbPool, dto::connexion::connexion_insertable::ConnexionInsertable,
    models::{connexion::Connexion, user::User},
};

pub trait ConnexionContract {
    /// create a new instance
    fn new(pool: &DbPool) -> Self
    where
        Self: Sized;

    fn create_connexion(&self, insertable: ConnexionInsertable) -> Connexion;

    fn get_user_last_connexion(&self, user: &User) -> Option<Connexion>;

    fn get_user_ip_connexion(&self, user: &User, test_ip: &String) -> Option<Connexion>;

    fn get_user_fingerprint_connexion(
        &self,
        user: &User,
        test_fingerprint: &String,
    ) -> Option<Connexion>;

}

pub trait ConnexionContractWithoutPool: ConnexionContract {
    fn new_without_pool() -> Self
    where
        Self: Sized;
}
