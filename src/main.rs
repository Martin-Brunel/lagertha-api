mod commands;
mod controlers;
mod core;
mod db;
mod dto;
mod enums;
mod guards;
mod models;
mod redis;
mod repositories;
mod schema;
mod services;
mod utils;
mod traits;
mod tests;
mod mocks;

extern crate rocket_cors;

use db::connect::establish_connection_pool;
use dotenv::dotenv;

use core::{api::CoreApi, cli::CoreCli};
use redis::{RedisClient, RedisConnexion};
use services::licence::{self, LicenceService};
use std::env;
use std::sync::Mutex;
use utils::cli::CLIUtils;

use crate::core::nodes_config::NodesConfig;
use crate::services::licence::License;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REDIS_POOL: RedisConnexion = {
        let client = RedisClient::get_connection();
        client
    };
}

lazy_static! {
    static ref LICENSE_VALID: Mutex<Option<License>> = Mutex::new(None);
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    CLIUtils::title();
    CLIUtils::separator();
    CLIUtils::empty_line();
    CLIUtils::empty_line();
    CLIUtils::separator();
    // data::fragments::connect().await;
    CLIUtils::empty_line();
    CLIUtils::separator();

    let license = LicenceService::new().await.is_valid();

    match license.clone() {
        None => {
            CLIUtils::write("The licence is not valid");
            CLIUtils::write("FREE mode")
        }
        Some(license) => {
            CLIUtils::write("Your licence is valid");
            CLIUtils::empty_line();
            CLIUtils::write(&format!(
                "{}",
                serde_json::to_string_pretty(&license).expect("Failed to serialize struct")
            ));
            CLIUtils::empty_line();
        }
    }
    CLIUtils::separator();

    {
        let mut license_valid = LICENSE_VALID.lock().unwrap();
        *license_valid = license;
    }

    let pool = establish_connection_pool();
    let nodes_config = match NodesConfig::new() {
        Err(e) => {
            println!("{:?}", e);
            panic!()
        }
        Ok(settings) => settings,
    };
    let args: Vec<String> = env::args().collect();
    match args.len() > 1 {
        true => CoreCli::exec(args, pool).await,
        false => CoreApi::launch(pool, nodes_config),
    }
}
