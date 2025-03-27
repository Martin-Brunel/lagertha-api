use rocket::figment::Figment;
use std::env;

pub struct CoreSettings;

impl CoreSettings {
    pub fn get() -> Figment {
        let tls_certs = env::var("TLS_CERT_PATH").expect("TLS_CERT_PATH must be set");
        let tls_key = env::var("TLS_KEY_PATH").expect("TLS_KEY_PATH must be set");

        let custom_config = rocket::Config::figment()
            .merge(("port", env::var("PORT").unwrap().parse::<u16>().unwrap()))
            .merge(("address", "0.0.0.0"))
            .merge((
                "workers",
                env::var("WORKERS").unwrap().parse::<u16>().unwrap(),
            ))
            .merge(("secret_key", env::var("SECRET_KEY").unwrap()))
            .merge(("tls.certs", tls_certs))
            .merge(("tls.key", tls_key));
        custom_config
    }
}
