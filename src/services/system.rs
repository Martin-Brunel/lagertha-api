use crate::{
    db::connect::DbPool,
    dto::system::system_information_output::SystemInformationOutput,
    models::{connexion::Connexion, user::User},
    repositories::{
        anonymous_sentinel::AnonymousSentinelRepository, application::ApplicationRepository,
        cluster::ClusterRepository, connexion::ConnexionRepository, sentinel::SentinelRepository,
        user::UserRepository,
    },
    traits::application::ApplicationContract,
};

use super::sentinel;

pub struct SystemService<T> {
    pool: DbPool,
    user_repository: UserRepository,
    application_repository: T,
    cluster_repository: ClusterRepository,
    sentinel_repository: SentinelRepository,
    anonymous_sentinel_repository: AnonymousSentinelRepository,
}

impl<T: ApplicationContract> SystemService<T> {
    pub fn new(pool: &DbPool, application_repository: T) -> Self {
        Self {
            pool: pool.clone(),
            user_repository: UserRepository::new(&pool),
            application_repository,
            cluster_repository: ClusterRepository::new(&pool),
            sentinel_repository: SentinelRepository::new(&pool),
            anonymous_sentinel_repository: AnonymousSentinelRepository::new(&pool),
        }
    }

    pub fn get_informations(&self) -> SystemInformationOutput {
        let users = self.user_repository.count_users().unwrap();
        let applications = self.application_repository.count_applications().unwrap();
        let sentinels = self.sentinel_repository.count_sentinels().unwrap();
        let anonymous_sentinels = self
            .anonymous_sentinel_repository
            .count_anonymous_sentinels()
            .unwrap();
        SystemInformationOutput::new(users, applications, sentinels, anonymous_sentinels)
    }
}
