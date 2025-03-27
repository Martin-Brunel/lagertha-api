use crate::db::connect::DbPool;

use super::{errors, log::LogHandler, nodes_config::NodesConfig};
use std::env;

use rocket::{Build, Rocket};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use super::{cors::CoreCORS, routes::CoreRoutes, settings::CoreSettings};

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
pub struct CoreApi;

impl CoreApi {
    pub fn launch(pool: DbPool, nodes_config: NodesConfig) -> Rocket<Build> {
        let mode = env::var("MODE").unwrap_or_else(|_| "prod".to_string());

        let mut building_rocket = rocket::custom(CoreSettings::get())
            .manage(pool)
            .manage(nodes_config)
            .register(
                "/",
                catchers![errors::notfound, errors::unprocessable_entity],
            )
            .mount("/", routes![all_options])
            .mount("/", CoreRoutes::get());

        if mode != "prod" {
            building_rocket = building_rocket.mount(
                "/",
                make_swagger_ui(&SwaggerUIConfig {
                    url: "../openapi.json".to_owned(),
                    ..Default::default()
                }),
            );
        }

        building_rocket = building_rocket.attach(CoreCORS).attach(LogHandler);
        building_rocket
    }
}
