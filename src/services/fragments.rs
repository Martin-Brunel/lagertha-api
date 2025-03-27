use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::Commands;
use sharks::{Share, Sharks};
use std::sync::Arc;
use std::{env, time::Instant};

use crate::core::nodes_config::{NodesConfig, RedisPools};

pub struct FragmentsService;

impl FragmentsService {
    /// Generates fragments from an encrypted key using Shamir's Secret Sharing scheme.
    ///
    /// # Arguments
    ///
    /// * `encrypted_key` - A string representing the encrypted key to be split.
    ///
    /// # Returns
    ///
    /// A list of fragments encoded in hexadecimal.
    pub fn generate_fragments(encrypted_key: String) -> Vec<String> {
        let trigger = env::var("FRAGMENTS_THRESHOLD")
            .unwrap_or_else(|_| format!("2"))
            .parse::<u8>()
            .unwrap();
        let number_of_fragments = env::var("FRAGMENTS_SHARES")
            .unwrap_or_else(|_| format!("3"))
            .parse::<usize>()
            .unwrap();
        let secret = encrypted_key.as_bytes();
        let sharks = Sharks(trigger);
        let fragment_generator = sharks.dealer(secret);
        fragment_generator
            .take(number_of_fragments)
            .map(|fragment| hex::encode(Vec::from(&fragment as &Share)))
            .collect()
    }

    /// Reconstructs the encrypted key from fragments using Shamir's Secret Sharing scheme.
    ///
    /// # Arguments
    ///
    /// * `fragments` - A list of fragments encoded in hexadecimal.
    ///
    /// # Returns
    ///
    /// An option containing the reconstructed encrypted key as a string, or `None` if the reconstruction fails.
    pub fn reconstruct_encrypted_key(fragments: Vec<String>) -> Option<String> {

        let threshold = env::var("FRAGMENTS_THRESHOLD")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u8>()
            .expect("Invalid threshold value");
        let sharks = Sharks(threshold);
        let shares: Vec<Share> = fragments
            .into_iter()
            .filter_map(|fragment| {
                hex::decode(fragment)
                    .ok()
                    .and_then(|bytes| Share::try_from(bytes.as_slice()).ok())
            })
            .collect();
        if shares.len() < threshold as usize {
            return None;
        }
        let res = sharks
            .recover(&shares)
            .ok()
            .map(|secret| String::from_utf8(secret).expect("Failed to convert secret to String"));

        res
    }

    pub fn save_fragments_to_nodes(
        fragments: Vec<String>,
        key_id: String,
        nodes_config: &NodesConfig,
    ) {
        for (index, fragment) in fragments.into_iter().enumerate() {
            let node_index = index % nodes_config.nodes.len();
            let node = &nodes_config.nodes[node_index];
            let client = redis::Client::open(node.clone().host).expect("Failed to connect redis");
            let mut conn = client.get_connection().unwrap();
            let _: () = redis::cmd("AUTH")
                .arg(node.clone().password)
                .query(&mut conn)
                .unwrap();
            let redis_key = format!("fragments:{}", key_id);
            let _: () = conn
                .set(&redis_key, fragment)
                .expect("Failed to save fragment to Redis");
        }
    }

    pub fn get_fragments_from_nodes(key_id: String, nodes_config: &NodesConfig) -> Vec<String> {
        let redis_key = format!("fragments:{}", key_id);
        let mut res = vec![];

        for node in nodes_config.nodes.clone() {
            let client = match redis::Client::open(node.clone().host) {
                Err(_) => continue,
                Ok(client) => client,
            };
            let mut conn = match client.get_connection() {
                Err(_) => continue,
                Ok(conn) => conn,
            };
            let _: () = match redis::cmd("AUTH")
                .arg(node.clone().password)
                .query(&mut conn)
            {
                Err(_) => continue,
                Ok(a) => a,
            };
            let fragment = conn.get(&redis_key);
            match fragment {
                Err(_) => continue,
                Ok(fragment) => res.push(fragment),
            }
        }
        
        res
    }

    pub fn delete_fragments_from_nodes(key_id: String, nodes_config: &NodesConfig) {
        let redis_key = format!("fragments:{}", key_id);
        for node in nodes_config.nodes.clone() {
            let client = redis::Client::open(node.clone().host).expect("Failed to connect redis");
            let mut conn = client.get_connection().unwrap();
            let _: () = redis::cmd("AUTH")
                .arg(node.clone().password)
                .query(&mut conn)
                .unwrap();
            let _: bool = conn.del(&redis_key).unwrap();
        }
    }
}
