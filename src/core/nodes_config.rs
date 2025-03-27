use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use serde::Deserialize;
use std::sync::Arc;

type RedisPool = Arc<Pool<RedisConnectionManager>>;
pub type RedisPools = Vec<RedisPool>;

#[derive(Debug, Deserialize, Clone)]
pub struct NodesConfig {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    pub host: String,
    pub password: String,
}

use config::{Config, ConfigError, File};

impl NodesConfig {
    pub fn new() -> Result<Self, ConfigError> {
        match Config::builder()
            .add_source(File::with_name("Fragments"))
            .build()
        {
            Err(e) => Err(e),
            Ok(config) => {
                let res = config.try_deserialize::<Self>();
                res
            }
        }
    }

    pub fn get_redis_pools(&self) -> RedisPools {
        let nodes = self.nodes.clone();
        let redis_pools = nodes
            .into_iter()
            .map(|node| {
                let manager = RedisConnectionManager::new(format!(
                    "{}?password={}",
                    node.host, node.password
                ))
                .expect("Connection manager error");
                let pool = Pool::builder()
                    .connection_timeout(std::time::Duration::from_secs(5))
                    .build(manager)
                    .expect("Failed to build pool");
                Arc::new(pool)
            })
            .collect();
        redis_pools
    }
}
