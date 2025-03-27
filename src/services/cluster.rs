use rocket::http::Status;
use uuid::Uuid;

use crate::{
    db::connect::DbPool,
    dto::{
        cluster::{
            cluster_anonymous_sentinels_input::ClusterAnonymousSentinelsInput,
            cluster_input::ClusterInput, cluster_insertable::ClusterInsertable,
            cluster_memberships_input::ClusterMembershipsInput,
            cluster_sentinels_input::ClusterSentinelsInput,
        },
        x_anonymous_sentinel_cluster::x_anonymous_sentinel_cluster_insertable::XAnonymousSentinelClusterInsertable,
        x_sentinel_cluster::x_sentinel_cluster_insertable::XSentinelClusterInsertable,
        x_user_cluster::x_user_cluster_insertable::XUserClusterInsertable,
    },
    enums::roles::Role,
    guards::security::Security,
    models::{cluster::Cluster, user::User},
    repositories::{
        anonymous_sentinel::AnonymousSentinelRepository, cluster::ClusterRepository,
        sentinel::SentinelRepository, user::UserRepository,
    },
    traits::{application::ApplicationContract, connexion::ConnexionContract},
};

pub struct ClusterService<T, D> {
    pool: DbPool,
    user_repository: UserRepository,
    application_repository: T,
    connexion_repository: D,
    cluster_repository: ClusterRepository,
    sentinel_repository: SentinelRepository,
    anonymous_sentinel_repository: AnonymousSentinelRepository,
}

impl<T: ApplicationContract, D: ConnexionContract> ClusterService<T, D> {
    pub fn new(pool: &DbPool, application_repository: T, connexion_repository: D) -> Self {
        Self {
            pool: pool.clone(),
            user_repository: UserRepository::new(&pool),
            application_repository,
            connexion_repository,
            cluster_repository: ClusterRepository::new(&pool),
            sentinel_repository: SentinelRepository::new(&pool),
            anonymous_sentinel_repository: AnonymousSentinelRepository::new(&pool),
        }
    }

    pub fn create(
        &self,
        input: ClusterInput,
        user_from: User,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let insertable = ClusterInsertable::new(
            input.name,
            input.description,
            user_from.application.unwrap(),
            user_from.id,
        );
        let cluster = self.cluster_repository.create_cluster(insertable);
        self.add_memberships_to_cluster(input.memberships, user_from, cluster.clone());
        Ok(cluster)
    }

    pub fn add_memberships(
        &self,
        cluster_uuid: Uuid,
        input: ClusterMembershipsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let memberships_list = input.memberships;
                self.add_memberships_to_cluster(memberships_list, user_from, cluster.clone());
                Ok(cluster)
            }
        }
    }

    fn add_memberships_to_cluster(
        &self,
        memberships_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
    ) {
        for member in memberships_list {
            // Pour chaque itération:
            let member_uuid = match Uuid::parse_str(&member) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };
            // - recuperer le user (id + application id)
            let user = match self
                .user_repository
                .get_by_id_and_app(&member_uuid, user_from.application.unwrap().clone())
            {
                None => continue,
                Some(user) => user,
            };
            match self
                .cluster_repository
                .get_user_cluster_memberships(&cluster, &user.id)
            {
                true => continue,
                false => {
                    let insertable =
                        XUserClusterInsertable::new(cluster.id, user.id, user_from.clone().id);
                    let _ = self.cluster_repository.add_user_to_cluster(insertable);
                }
            }
        }
    }

    pub fn remove_memberships(
        &self,
        cluster_uuid: Uuid,
        input: ClusterMembershipsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let memberships_list = input.memberships;
                self.remove_memberships_from_cluster(memberships_list, user_from, cluster.clone());
                Ok(cluster)
            }
        }
    }

    fn remove_memberships_from_cluster(
        &self,
        memberships_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
    ) {
        for member in memberships_list {
            let member_uuid = match Uuid::parse_str(&member) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };
            let user = match self
                .user_repository
                .get_by_id_and_app(&member_uuid, user_from.application.unwrap().clone())
            {
                None => continue,
                Some(user) => user,
            };
            let _ = self
                .cluster_repository
                .remove_user_from_cluster(&user, &cluster, &user_from);
        }
    }

    pub fn delete(
        &self,
        cluster_uuid: Uuid,
        authorised: Security,
    ) -> Result<(), (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let _ = self.cluster_repository.delete(cluster_uuid, user_from);
                Ok(())
            }
        }
    }

    pub fn add_sentinels(
        &self,
        cluster_uuid: Uuid,
        input: ClusterSentinelsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let is_admin = authorised.check_roles(Role::ADMIN);
                let sentinels_list = input.sentinels;
                self.add_sentinels_to_cluster(sentinels_list, user_from, cluster.clone(), is_admin);
                Ok(cluster)
            }
        }
    }

    fn add_sentinels_to_cluster(
        &self,
        sentinels_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
        is_admin: bool,
    ) {
        for sentinel in sentinels_list {
            // Pour chaque itération:
            let sentinel_uuid = match Uuid::parse_str(&sentinel) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };

            let sentinel = match self
                .sentinel_repository
                .get_sentinel_by_id(&sentinel_uuid, &user_from)
            {
                None => continue,
                Some(sentinel) => {
                    match is_admin || sentinel.created_by_id.unwrap() == user_from.id {
                        false => continue,
                        true => sentinel,
                    }
                }
            };

            match self
                .cluster_repository
                .get_user_cluster_sentinels(&cluster, &sentinel.id)
            {
                true => continue,
                false => {
                    let insertable = XSentinelClusterInsertable::new(
                        cluster.id,
                        sentinel.id,
                        user_from.clone().id,
                    );
                    let _ = self.cluster_repository.add_sentinel_to_cluster(insertable);
                }
            }
        }
    }

    pub fn add_anonymous_sentinels(
        &self,
        cluster_uuid: Uuid,
        input: ClusterAnonymousSentinelsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let is_admin = authorised.check_roles(Role::ADMIN);
                let anonymous_sentinels_list = input.anonymous_sentinels;
                self.add_anonymous_sentinels_to_cluster(
                    anonymous_sentinels_list,
                    user_from,
                    cluster.clone(),
                    is_admin,
                );
                Ok(cluster)
            }
        }
    }

    fn add_anonymous_sentinels_to_cluster(
        &self,
        anonymous_sentinels_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
        is_admin: bool,
    ) {
        for anonymous_sentinel in anonymous_sentinels_list {
            // Pour chaque itération:
            let anonymous_sentinel_uuid = match Uuid::parse_str(&anonymous_sentinel) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };

            let anonymous_sentinel = match self
                .anonymous_sentinel_repository
                .get_anonymous_sentinel_by_id(&anonymous_sentinel_uuid, &user_from)
            {
                None => continue,
                Some(sentinel) => {
                    match is_admin || sentinel.created_by_id.unwrap() == user_from.id {
                        false => continue,
                        true => sentinel,
                    }
                }
            };

            match self
                .cluster_repository
                .get_user_cluster_anonymous_sentinels(&cluster, &anonymous_sentinel.id)
            {
                true => continue,
                false => {
                    let insertable = XAnonymousSentinelClusterInsertable::new(
                        cluster.id,
                        anonymous_sentinel.id,
                        user_from.clone().id,
                    );
                    let _ = self
                        .cluster_repository
                        .add_anonymous_sentinel_to_cluster(insertable);
                }
            }
        }
    }

    pub fn remove_sentinels(
        &self,
        cluster_uuid: Uuid,
        input: ClusterSentinelsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let sentinels_list = input.sentinels;
                let is_admin = authorised.check_roles(Role::ADMIN);
                self.remove_sentinels_from_cluster(
                    sentinels_list,
                    user_from,
                    cluster.clone(),
                    is_admin,
                );
                Ok(cluster)
            }
        }
    }

    fn remove_sentinels_from_cluster(
        &self,
        sentinels_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
        is_admin: bool,
    ) {
        for sentinel in sentinels_list {
            let sentinel_uuid = match Uuid::parse_str(&sentinel) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };
            let sentinel = match self
                .sentinel_repository
                .get_sentinel_by_id(&sentinel_uuid, &user_from)
            {
                None => continue,
                Some(sentinel) => {
                    match is_admin || sentinel.created_by_id.unwrap() == user_from.id {
                        false => continue,
                        true => sentinel,
                    }
                }
            };

            let _ = self
                .cluster_repository
                .remove_sentinel_from_cluster(&sentinel, &cluster, &user_from);
        }
    }

    pub fn remove_anonymous_sentinels(
        &self,
        cluster_uuid: Uuid,
        input: ClusterAnonymousSentinelsInput,
        authorised: Security,
    ) -> Result<Cluster, (Status, Option<&str>)> {
        let user_from = authorised.user.clone();
        let cluster = match self
            .cluster_repository
            .get_by_id_and_application(&cluster_uuid, &user_from.application.unwrap())
        {
            None => return Err((Status::NotFound, None)),
            Some(cluster) => cluster,
        };
        match authorised.check_roles(Role::ADMIN) || cluster.created_by_id == Some(user_from.id) {
            false => Err((Status::Forbidden, None)),
            true => {
                let anonymous_sentinels_list = input.anonymous_sentinels;
                let is_admin = authorised.check_roles(Role::ADMIN);
                self.remove_anonymous_sentinels_from_cluster(
                    anonymous_sentinels_list,
                    user_from,
                    cluster.clone(),
                    is_admin,
                );
                Ok(cluster)
            }
        }
    }

    fn remove_anonymous_sentinels_from_cluster(
        &self,
        anonymous_sentinels_list: Vec<String>,
        user_from: User,
        cluster: Cluster,
        is_admin: bool,
    ) {
        for sentinel in anonymous_sentinels_list {
            let sentinel_uuid = match Uuid::parse_str(&sentinel) {
                Err(_) => continue,
                Ok(uuid) => uuid,
            };
            let anonymous_sentinel = match self
                .anonymous_sentinel_repository
                .get_anonymous_sentinel_by_id(&sentinel_uuid, &user_from)
            {
                None => continue,
                Some(sentinel) => {
                    match is_admin || sentinel.created_by_id.unwrap() == user_from.id {
                        false => continue,
                        true => sentinel,
                    }
                }
            };

            let _ = self
                .cluster_repository
                .remove_anonymous_sentinel_from_cluster(&anonymous_sentinel, &cluster, &user_from);
        }
    }

    pub fn get_cluster_users(
        &self,
        cluster_uuid: &Uuid,
        user_from: User,
        page_size: usize,
        page: usize,
    ) -> Result<(Vec<User>, i64), Status> {
        match self.cluster_repository.get_cluster_users(
            cluster_uuid,
            user_from.application,
            page,
            page_size,
        ) {
            None => Err(Status::NotFound),
            Some((users, count)) => Ok((users, count)),
        }
    }
}
