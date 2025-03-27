use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let post_gres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let post_gres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let post_gres_db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let post_gres_db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        post_gres_user, post_gres_password, post_gres_db_host, post_gres_db_name
    );
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
