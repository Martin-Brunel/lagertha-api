use crate::services::mail::MailService;
use crate::{
    models::application::Application,
    schema::users,
    utils::{
        code::{self, generate_base32_key},
        crypto::Crypto,
        password::PasswordUtils,
        pq_kyber::PQKyber,
    },
};
use chrono::{DateTime, Utc};
use diesel::prelude::Insertable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user_input::UserInput;
use super::user_public_input::UserPublicInput;

// use super::application_input::ApplicationInput;

#[derive(Deserialize, Serialize, Debug, Insertable)]
#[diesel(table_name = users)]
pub struct UserInsertable {
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub twofa_code: String,
    pub is_2fa_activated: bool,
    pub login: String,
    pub roles: Vec<String>,
    pub password: Option<String>,
    pub full_text_search: String,
    pub kyber_secret_key: String,
    pub kyber_public_key: String,
    pub iv: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<Uuid>,
    pub updated_by_id: Option<Uuid>,
    pub deleted_by_id: Option<Uuid>,
    pub refresh_token: Option<String>,
    pub application: Option<i32>,
    pub restricted_ip: Option<Vec<String>>,
    pub is_validated: bool,
    pub validation_code: Option<String>,
    pub validation_tries: i32,
}

impl UserInsertable {
    pub async fn new(input: UserInput, application: Application) -> Self {
        let hash_password = match input.password {
            None => None,
            Some(password) => Some(PasswordUtils::hash_password(password)),
        };
        let roles = match input.is_admin {
            true => vec![String::from("ROLE_ADMIN")],
            false => vec![String::from("ROLE_USER")],
        };
        let (public, secret) = PQKyber::generate_key_pair();
        let pq_sk = hex::encode(secret);
        let pq_pk = hex::encode(public);
        let iv = Crypto::generate_unique_iv();
        let pq_sk_encrypted = PQKyber::encrypt_key(pq_sk, iv.clone());
        let pq_pk_encrypted = PQKyber::encrypt_key(pq_pk, iv.clone());
        UserInsertable {
            email: input.email.to_string(),
            password: hash_password,
            firstname: input.firstname.to_string(),
            lastname: input.lastname.to_string(),
            login: input.login.clone(),
            is_validated: true,
            validation_tries: 0,
            validation_code: None,
            twofa_code: generate_base32_key(160),
            is_2fa_activated: false,
            full_text_search: format!(
                "{} {} {} {}",
                input.email, input.login, input.firstname, input.lastname
            ),
            kyber_secret_key: pq_sk_encrypted,
            kyber_public_key: pq_pk_encrypted,
            iv,
            restricted_ip: None,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            roles,
            application: Some(application.id),
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
        }
    }

    pub fn new_command_line(
        email: String,
        firstname: String,
        lastname: String,
        app_id: i32,
        login: &String,
        password: &String,
        is_system: bool,
    ) -> Self {
        let roles = match is_system {
            true => vec![String::from("ROLE_SUPER_ADMIN")],
            false => vec![String::from("ROLE_ADMIN")],
        };
        let hash_password = PasswordUtils::hash_password(password.clone());
        let (public, secret) = PQKyber::generate_key_pair();
        let pq_sk = hex::encode(secret);
        let pq_pk = hex::encode(public);
        let iv = Crypto::generate_unique_iv();
        let pq_sk_encrypted = PQKyber::encrypt_key(pq_sk, iv.clone());
        let pq_pk_encrypted = PQKyber::encrypt_key(pq_pk, iv.clone());

        UserInsertable {
            email: email.clone(),
            password: Some(hash_password),
            firstname: firstname.clone(),
            lastname: lastname.clone(),
            login: login.clone(),
            validation_tries: 0,
            twofa_code: generate_base32_key(160),
            is_2fa_activated: false,
            full_text_search: format!("{} {} {} {}", email, login, firstname, lastname),
            is_deleted: false,
            is_validated: true,
            validation_code: None,
            created_at: Utc::now(),
            updated_at: None,
            roles,
            restricted_ip: None,
            application: Some(app_id),
            kyber_secret_key: pq_sk_encrypted,
            kyber_public_key: pq_pk_encrypted,
            iv,
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
        }
    }

    pub async fn new_public(input: UserPublicInput, application: &Application) -> Self {
        let hash_password = PasswordUtils::hash_password(input.password);
        let roles = vec![String::from("ROLE_USER")];
        let (public, secret) = PQKyber::generate_key_pair();
        let pq_sk = hex::encode(secret);
        let pq_pk = hex::encode(public);
        let iv = Crypto::generate_unique_iv();
        let pq_sk_encrypted = PQKyber::encrypt_key(pq_sk, iv.clone());
        let pq_pk_encrypted = PQKyber::encrypt_key(pq_pk, iv.clone());
        let validation_code = code::generate_reset_code();
        MailService::send_validation_code(&input.email, &input.login, &validation_code).await;
        UserInsertable {
            email: input.email.to_string(),
            password: Some(hash_password),
            firstname: input.firstname.to_string(),
            lastname: input.lastname.to_string(),
            login: input.login.clone(),
            is_2fa_activated: false,
            full_text_search: format!(
                "{} {} {} {}",
                input.email, input.login, input.firstname, input.lastname
            ),
            twofa_code: generate_base32_key(160),
            kyber_secret_key: pq_sk_encrypted,
            kyber_public_key: pq_pk_encrypted,
            iv,
            is_validated: false,
            validation_tries: 0,
            validation_code: Some(PasswordUtils::hash_password(validation_code)),
            restricted_ip: None,
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: None,
            roles,
            application: Some(application.id),
            deleted_at: None,
            created_by_id: None,
            updated_by_id: None,
            deleted_by_id: None,
            refresh_token: None,
        }
    }
}
