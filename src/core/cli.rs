use regex::Regex;
use rocket::{Build, Rocket};
use std::process::{self};

use crate::{
    commands::{
        create_application::CommandCreateApplication, hsm_init::CommandHsmInit, init::CommandInit,
    },
    db::connect::DbPool,
};

pub struct CoreCli;

impl CoreCli {
    pub async fn exec(args: Vec<String>, pool: DbPool) -> Rocket<Build> {
        let mut _build = rocket::build();
        let command = &*args[1];
        let _ = match command {
            "init" => CommandInit::exec(pool).await,
            "hsm_init" => CommandHsmInit::exec().await,
            "create_application" => {
                let email_regex =
                    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
                match &*args[2] {
                    "" => println!("missing name argument"),
                    _ => match email_regex.is_match(&*args[3]) {
                        true => CommandCreateApplication::exec(pool, &args[2], &args[3]).await,
                        false => println!("missing or bad email argument"),
                    },
                };
            }
            _ => {
                println!("bad_request");
            }
        };
        process::exit(0x0100);
    }
}
