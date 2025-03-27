use dotenv::dotenv;
use std::env;
use redis::Connection;

extern crate redis;
pub struct RedisClient;

pub type RedisConnexion = Connection;

impl RedisClient {
    pub fn get_connection() -> RedisConnexion {
        dotenv().ok();
        let client = redis::Client::open(env::var("REDIS_URL").unwrap()).expect("Failed to connect redis");
        let mut con = client.get_connection().unwrap();
        let _: () = redis::cmd("AUTH")
            .arg(env::var("REDIS_PASSWORD").unwrap())
            .query(&mut con)
            .unwrap();
        con
    }
}
