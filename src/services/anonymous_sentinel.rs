use mail_send::mail_auth::hickory_resolver::proto::rr::dnssec::Private;
use rocket::http::Status;
use uuid::Uuid;

use crate::{
    core::nodes_config::NodesConfig,
    db::connect::DbPool,
    dto::{
        anonymous_sentinel::{
            anonymous_sentinel_insertable::AnonymousSentinelInsertable,
            anonymous_sentinel_public_input::AnonymousSentinelPublicInput,
        },
        sentinel::sentinel_input::SentinelInput,
        x_anonymous_sentinel_cluster::x_anonymous_sentinel_cluster_insertable::XAnonymousSentinelClusterInsertable,
    },
    models::{anonymous_sentinel::AnonymousSentinel, user::User},
    repositories::{anonymous_sentinel::AnonymousSentinelRepository, cluster::ClusterRepository},
    traits::application::ApplicationContract,
    utils::{crypto::Crypto, pq_kyber::PQKyber},
    LICENSE_VALID,
};

use super::fragments::FragmentsService;

pub struct AnonymousSentinelService<T> {
    nodes_config: NodesConfig,
    anonymous_sentinel_repository: AnonymousSentinelRepository,
    cluster_repository: ClusterRepository,
    application_repository: T,
}

impl<T: ApplicationContract> AnonymousSentinelService<T> {
    pub fn new(pool: &DbPool, application_repository: T, nodes_config: &NodesConfig) -> Self {
        Self {
            nodes_config: nodes_config.clone(),
            anonymous_sentinel_repository: AnonymousSentinelRepository::new(&pool),
            cluster_repository: ClusterRepository::new(&pool),
            application_repository,
        }
    }

    pub fn create(
        &self,
        input: SentinelInput,
        user_from: User,
    ) -> Result<(AnonymousSentinel, String), (Status, Option<&str>)> {
        let license_valid = LICENSE_VALID.lock().unwrap();
        let key_size = match license_valid.clone() {
            None => 512,
            Some(license) => match license.mode != format!("Entreprise") {
                true => 512,
                false => 1024,
            },
        };
        let (public, secret) = match key_size.clone() == 1024 {
            true => {
                let (pk, sk) = PQKyber::generate_key_pair();
                (hex::encode(pk), hex::encode(sk))
            }
            false => {
                let (pk, sk) = PQKyber::generate_key_pair_512();
                (hex::encode(pk), hex::encode(sk))
            }
        };

        let iv = Crypto::generate_unique_iv();

        let public = PQKyber::encrypt_key(public, iv.clone());
        let secret_encode = PQKyber::encrypt_key(secret.clone(), iv.clone());

        let fragments = FragmentsService::generate_fragments(secret_encode.clone());
        let sum = Crypto::key_sum(&secret_encode);

        let insertable = AnonymousSentinelInsertable::new(
            iv,
            sum,
            public,
            user_from.application.unwrap(),
            Some(user_from.id),
            key_size,
        );
        let sentinel = self
            .anonymous_sentinel_repository
            .create_anonymous_sentinel(insertable);
        FragmentsService::save_fragments_to_nodes(
            fragments,
            sentinel.id.to_string(),
            &self.nodes_config,
        );

        let clusters = input.clusters;
        for cluster_id in clusters {
            let cluster_uuid = match Uuid::parse_str(&cluster_id) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };
            match self
                .cluster_repository
                .get_user_cluster_by_id(&cluster_uuid, &user_from)
            {
                None => continue,
                Some(cluster) => {
                    let insertable = XAnonymousSentinelClusterInsertable::new(
                        cluster.id,
                        sentinel.id.clone(),
                        user_from.id.clone(),
                    );
                    self.cluster_repository
                        .add_anonymous_sentinel_to_cluster(insertable);
                }
            }
        }

        let _ = self
            .application_repository
            .increment_keys(&user_from.application.unwrap());
        Ok((sentinel, secret))
    }

    pub fn create_public(
        &self,
        input: AnonymousSentinelPublicInput,
    ) -> Result<AnonymousSentinel, (Status, Option<&str>)> {
        let license_valid = LICENSE_VALID.lock().unwrap();
        let key_size = match license_valid.clone() {
            None => 512,
            Some(license) => match license.mode != format!("Entreprise") {
                true => 512,
                false => 1024,
            },
        };
        let (public, secret) = match key_size.clone() == 1024 {
            true => {
                let (pk, sk) = PQKyber::generate_key_pair();
                (hex::encode(pk), hex::encode(sk))
            }
            false => {
                let (pk, sk) = PQKyber::generate_key_pair_512();
                (hex::encode(pk), hex::encode(sk))
            }
        };

        let iv = Crypto::generate_unique_iv();

        let public_encode = PQKyber::encrypt_key(public, iv.clone());
        let secret_encode = PQKyber::encrypt_key(secret, iv.clone());

        let fragments = FragmentsService::generate_fragments(secret_encode.clone());
        let sum = Crypto::key_sum(&secret_encode);

        let insertable = AnonymousSentinelInsertable::new(
            iv,
            sum,
            public_encode,
            input.application_id,
            None,
            key_size,
        );
        let sentinel = self
            .anonymous_sentinel_repository
            .create_anonymous_sentinel(insertable);
        FragmentsService::save_fragments_to_nodes(
            fragments,
            sentinel.id.to_string(),
            &self.nodes_config,
        );
        let _ = self
            .application_repository
            .increment_keys(&input.application_id);
        Ok(sentinel)
    }

    pub fn get_public(
        &self,
        sentinel_uuid: Uuid,
    ) -> Result<AnonymousSentinel, (Status, Option<&str>)> {
        match self
            .anonymous_sentinel_repository
            .get_public_anonymous_sentinel_by_id(&sentinel_uuid)
        {
            None => Err((Status::NotFound, None)),
            Some(anonymous_sentinel) => Ok(anonymous_sentinel),
        }
    }

    pub fn get_by_id(
        &self,
        sentinel_uuid: Uuid,
        user_from: User,
    ) -> Result<(AnonymousSentinel, String), (Status, Option<&str>)> {
        match self
            .anonymous_sentinel_repository
            .get_anonymous_sentinel_by_id(&sentinel_uuid, &user_from)
        {
            None => Err((Status::NotFound, None)),
            Some(anonymous_sentinel) => {
                let fragments = FragmentsService::get_fragments_from_nodes(
                    anonymous_sentinel.clone().id.to_string(),
                    &self.nodes_config,
                );
                match FragmentsService::reconstruct_encrypted_key(fragments) {
                    None => Err((Status::NotFound, None)),
                    Some(encrypted_key) => match anonymous_sentinel.check(encrypted_key.clone()) {
                        Err(e) => Err((Status::NotAcceptable, Some(e))),
                        Ok(valid_sentinel) => Ok((
                            valid_sentinel.clone(),
                            PQKyber::decrypt_key(encrypted_key, valid_sentinel.iv),
                        )),
                    },
                }
            }
        }
    }

    pub fn delete_one(
        &self,
        sentinel_uuid: Uuid,
        user_from: User,
        is_admin: bool,
    ) -> Result<(), (Status, Option<&str>)> {
        match is_admin {
            true => match self
                .anonymous_sentinel_repository
                .delete_sentinel_by_id_admin(&sentinel_uuid, &user_from)
            {
                Err(_) => return Err((Status::NotFound, None)),
                Ok(_) => {}
            },
            false => match self
                .anonymous_sentinel_repository
                .delete_sentinel_by_id_user(&sentinel_uuid, &user_from)
            {
                Err(_) => return Err((Status::NotFound, None)),
                Ok(_) => {}
            },
        };
        FragmentsService::delete_fragments_from_nodes(
            sentinel_uuid.to_string(),
            &self.nodes_config,
        );
        let _ = self
            .application_repository
            .decrement_keys(&user_from.application.unwrap());
        Ok(())
    }
}
