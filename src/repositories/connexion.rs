use diesel::prelude::*;

use crate::dto::connexion::connexion_insertable::ConnexionInsertable;
use crate::models::connexion::Connexion;
use crate::models::user::User;
use crate::schema::connexions::dsl::*;
use crate::schema::connexions::user_id;
use crate::traits::connexion::ConnexionContract;
use crate::{db::connect::DbPool, schema::connexions};

pub struct ConnexionRepository {
    pool: DbPool,
}

impl ConnexionContract for ConnexionRepository {
    fn new(pool: &DbPool) -> Self {
        Self { pool: pool.clone() }
    }

    fn create_connexion(&self, insertable: ConnexionInsertable) -> Connexion {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        diesel::insert_into(connexions::table)
            .values(&insertable)
            .returning(Connexion::as_returning())
            .get_result(&mut conn)
            .expect("failed to insert application")
    }

    fn get_user_last_connexion(&self, user: &User) -> Option<Connexion> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match connexions
            .filter(user_id.eq(user.id))
            .filter(is_deleted.eq(false))
            .first::<Connexion>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(app) => app,
        }
    }

    fn get_user_ip_connexion(&self, user: &User, test_ip: &String) -> Option<Connexion> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match connexions
            .filter(user_id.eq(user.id))
            .filter(is_deleted.eq(false))
            .filter(ip.eq(test_ip))
            .first::<Connexion>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(app) => app,
        }
    }

    fn get_user_fingerprint_connexion(
        &self,
        user: &User,
        test_fingerprint: &String,
    ) -> Option<Connexion> {
        let mut conn = self.pool.get().expect("Fail to get a db connexion");
        match connexions
            .filter(
                user_id
                    .eq(user.id)
                    .and(is_deleted.eq(false))
                    .and(fingerprint.eq(test_fingerprint)),
            )
            .first::<Connexion>(&mut conn)
            .optional()
        {
            Err(_) => None,
            Ok(app) => app,
        }
    }
}
