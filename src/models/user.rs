use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, PartialEq, Queryable, Selectable, Clone, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub twofa_code: String,
    pub is_2fa_activated: bool,
    pub login: String,
    pub roles: Vec<Option<String>>,
    pub password: Option<String>,
    pub full_text_search: String,
    pub kyber_secret_key: String,
    pub kyber_public_key: String,
    pub iv: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by_id: Option<uuid::Uuid>,
    pub updated_by_id: Option<uuid::Uuid>,
    pub deleted_by_id: Option<uuid::Uuid>,
    pub refresh_token: Option<String>,
    pub application: Option<i32>,
    pub restricted_ip: Vec<Option<String>>,
    pub is_validated: bool,
    pub validation_code: Option<String>,
    pub validation_tries: i32,
    pub forget_code_delay: Option<DateTime<Utc>>
}

impl User {
    pub fn validation(&self) -> Self {
        match self.is_validated {
            true => self.clone(),
            false => {
                Self {
                    roles: vec![Some(String::from("ROLE_VALIDATION"))],
                    ..self.clone()
                }
            }
        }
    }
}