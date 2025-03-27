use rocket::http::Status;
use uuid::Uuid;

use crate::{
    core::nodes_config::NodesConfig,
    db::connect::DbPool,
    dto::{
        sentinel::{sentinel_input::SentinelInput, sentinel_insertable::SentinelInsertable},
        x_sentinel_cluster::x_sentinel_cluster_insertable::XSentinelClusterInsertable,
    },
    models::{sentinel::Sentinel, user::User},
    repositories::{
        application::ApplicationRepository, cluster::ClusterRepository,
        sentinel::SentinelRepository,
    },
    services::fragments::{self, FragmentsService},
    traits::application::ApplicationContract,
    utils::crypto::Crypto,
    LICENSE_VALID,
};

pub struct SentinelService<T> {
    nodes_config: NodesConfig,
    sentinel_repository: SentinelRepository,
    cluster_repository: ClusterRepository,
    application_repository: T,
}

impl<T: ApplicationContract> SentinelService<T> {
    pub fn new(pool: &DbPool, application_repository: T, nodes_config: &NodesConfig) -> Self {
        Self {
            nodes_config: nodes_config.clone(),
            sentinel_repository: SentinelRepository::new(&pool),
            cluster_repository: ClusterRepository::new(&pool),
            application_repository,
        }
    }

    pub fn create(
        &self,
        input: SentinelInput,
        user_from: User,
    ) -> Result<(Sentinel, String), (Status, Option<&str>)> {
        let license_valid = LICENSE_VALID.lock().unwrap();
        let key_size = match license_valid.clone() {
            None => 128,
            Some(license) => match license.mode != format!("Entreprise") {
                true => 128,
                false => 256,
            },
        };
        let key = match key_size.clone() == 256 {
            true => Crypto::generate_aes_256_key(),
            false => Crypto::generate_aes_128_key(),
        };

        let iv = Crypto::generate_unique_iv();
        let encrypted = Crypto::encrypt(key.clone(), iv.clone());
        let fragments = FragmentsService::generate_fragments(encrypted.clone());
        let sum = Crypto::key_sum(&encrypted);
        let insertable = SentinelInsertable::new(
            iv,
            sum,
            user_from.application.unwrap(),
            user_from.id,
            key_size,
        );
        let sentinel = self.sentinel_repository.create_sentinel(insertable);
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
                    let insertable = XSentinelClusterInsertable::new(
                        cluster.id,
                        sentinel.id.clone(),
                        user_from.id.clone(),
                    );
                    self.cluster_repository.add_sentinel_to_cluster(insertable);
                }
            }
        }
        let _ = self
            .application_repository
            .increment_keys(&user_from.application.unwrap());
        Ok((sentinel, key))
    }

    pub fn get_by_id(
        &self,
        sentinel_uuid: Uuid,
        user_from: User,
    ) -> Result<(Sentinel, String), (Status, Option<&str>)> {
        match self
            .sentinel_repository
            .get_sentinel_by_id(&sentinel_uuid, &user_from)
        {
            None => Err((Status::NotFound, None)),
            Some(sentinel) => {
                let fragments = FragmentsService::get_fragments_from_nodes(
                    sentinel.clone().id.to_string(),
                    &self.nodes_config,
                );
                match FragmentsService::reconstruct_encrypted_key(fragments) {
                    None => Err((Status::NotFound, None)),
                    Some(encrypted_key) => match sentinel.check(encrypted_key.clone()) {
                        Err(e) => Err((Status::NotAcceptable, Some(e))),
                        Ok(valid_sentinel) => Ok((
                            valid_sentinel.clone(),
                            Crypto::decrypt(encrypted_key, valid_sentinel.iv),
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
                .sentinel_repository
                .delete_sentinel_by_id_admin(&sentinel_uuid, &user_from)
            {
                Err(_) => return Err((Status::NotFound, None)),
                Ok(_) => {}
            },
            false => match self
                .sentinel_repository
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
