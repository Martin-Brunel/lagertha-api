use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let post_gres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let post_gres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let post_gres_db_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let post_gres_db_name = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let database_url = format!(
        "postgres://{}:{}@{}/{}",
        post_gres_user, post_gres_password, post_gres_db_host, post_gres_db_name
    );
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(2) // Ajustez en fonction de vos besoins et capacités
        .build(manager)
        .expect("Failed to create pool.");

    pool
}