use std::env;

use crate::core::errors::ErrorObject;
use crate::db::connect::{establish_connection_pool, DbPool};
use crate::enums::roles::Role;
use crate::models::user::User;
use crate::redis::RedisClient;
use crate::{repositories::user::UserRepository, utils::jwt::Jwt};
use dotenv::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use redis::Commands;
use rocket::{request::Outcome, Request};
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::RequestHeaderInput,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Security {
    pub user: User,
}

impl Security {
    pub fn new(user: &User) -> Self {
        Self { user: user.clone() }
    }

    pub fn check_roles(&self, require_role: Role) -> bool {
        match require_role {
            Role::ADMIN => self.user.roles.iter().any(|x| match x {
                None => false,
                Some(x) => x.contains("ROLE_ADMIN"),
            }),
            Role::USER => self.user.roles.iter().any(|x| match x {
                None => false,
                Some(x) => x.contains("ROLE_USER"),
            }),
            Role::SUPERADMIN => self.user.roles.iter().any(|x| match x {
                None => false,
                Some(x) => x.contains("ROLE_SUPER_ADMIN"),
            }),
            Role::VALIDATION => self.user.roles.iter().any(|x| match x {
                None => false,
                Some(x) => x.contains("ROLE_VALIDATION"),
            }),
        }
    }

    pub fn check_headers(
        user: &User,
        device_id: Option<String>,
        req: &Request<'_>,
    ) -> Outcome<Self, ErrorObject> {
        let device_id = match device_id {
            None => {
                return Outcome::Error((
                    rocket::http::Status::Forbidden,
                    ErrorObject {
                        code: 403,
                        message: String::from("Wrong anti-replay headers"),
                    },
                ))
            }
            Some(device_id) => device_id,
        };
        let user_agent_headers = req.headers().get_one("User-Agent").map(|ua| ua.to_string());
        let nonce_headers = req.headers().get_one("X-NONCE").map(|ua| ua.to_string());
        let fingerprint_headers = req
            .headers()
            .get_one("X-FINGERPRINT")
            .map(|ua| ua.to_string());
        let nonce = match nonce_headers {
            None => {
                return Outcome::Error((
                    rocket::http::Status::Forbidden,
                    ErrorObject {
                        code: 403,
                        message: String::from("Wrong anti-replay headers"),
                    },
                ))
            }
            Some(nonce) => nonce,
        };
        let fingerprint = match fingerprint_headers {
            None => {
                return Outcome::Error((
                    rocket::http::Status::Forbidden,
                    ErrorObject {
                        code: 403,
                        message: String::from("Wrong anti-replay headers"),
                    },
                ))
            }
            Some(fingerprint) => fingerprint,
        };
        let user_agent = match user_agent_headers {
            None => format!(""),
            Some(user_agent) => user_agent,
        };
        let mut redis = RedisClient::get_connection();
        let exists: bool = redis.exists(nonce.clone()).unwrap();
        if exists {
            return Outcome::Error((
                rocket::http::Status::Forbidden,
                ErrorObject {
                    code: 403,
                    message: String::from("Wrong anti-replay headers"),
                },
            ));
        }
        let to_hash = &format!("{}{}", fingerprint, user_agent);
        let mut hasher = Sha256::new();
        hasher.update(to_hash);
        let device_sha = format!("{:x}", hasher.finalize());
        match device_sha == device_id {
            false => Outcome::Error((
                rocket::http::Status::Forbidden,
                ErrorObject {
                    code: 403,
                    message: String::from("Wrong anti-replay headers"),
                },
            )),
            true => {
                let token_duration = env::var("JWT_TOKEN_DURATION")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let _: bool = redis
                    .set_ex(nonce, "", token_duration)
                    .expect("failed to insert in redis");
                Outcome::Success(Self::new(user))
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for Security {
    type Error = ErrorObject;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorObject> {
        dotenv().ok();
        let pool: DbPool = establish_connection_pool();
        let user_repository = UserRepository::new(&pool);
        let mut keys = req.headers().get("Authorization");
        let key = keys.next();
        let outcome_error = Outcome::Error((
            rocket::http::Status::Forbidden,
            ErrorObject {
                code: 403,
                message: String::from("No token found"),
            },
        ));
        let jwt = match key {
            None => return outcome_error,
            Some(key) => key.replace("Bearer ", ""),
        };
        match decode::<Jwt>(
            jwt.as_ref(),
            &DecodingKey::from_rsa_pem(include_bytes!("../../ssh_keys/public.key")).unwrap(),
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(token) => {
                if token.claims.is_refresh {
                    return outcome_error;
                }
                match user_repository
                    .get_by_login_and_app(&token.claims.login, token.claims.application_id)
                {
                    None => outcome_error,
                    Some(user) => {
                        let mode = env::var("MODE").unwrap_or_else(|_| format!("prod"));
                        let user = user.validation();
                        match mode.as_str() {
                            "dev" => {
                                Outcome::Success(Self::new(&user))
                            }
                            _ => Security::check_headers(&user, token.claims.device_id, req),
                        }
                    }
                }
            }
            Err(_e) => outcome_error,
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for Security {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Value for the Authorization header parameter".to_owned()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
