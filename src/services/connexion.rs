use rocket::http::Status;

use crate::{
    dto::connexion::connexion_insertable::ConnexionInsertable,
    models::{connexion::Connexion, user::User},
    traits::connexion::ConnexionContract,
};

pub struct ConnexionService<T> {
    connexion_repository: T,
}

impl<T: ConnexionContract> ConnexionService<T> {
    pub fn new(connexion_repository: T) -> Self {
        Self {
            connexion_repository,
        }
    }

    pub fn create_connexion(
        &self,
        ip: &String,
        user_agent: &String,
        fingerprint: &String,
        user: &User,
    ) -> Connexion {
        let insertable =
            ConnexionInsertable::new(user.id, ip.clone(), fingerprint.clone(), user_agent.clone());
        let connexion = self.connexion_repository.create_connexion(insertable);
        connexion
    }
}
