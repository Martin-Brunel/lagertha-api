use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use std::env;

pub struct CoreCORS;

#[rocket::async_trait]
impl Fairing for CoreCORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let domains = env::var("CORS_ALLOWED_DOMAINS").unwrap();
        for domain in domains.split(";") {
            response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                domain.to_owned(),
            ));
        }
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, DELETE, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
