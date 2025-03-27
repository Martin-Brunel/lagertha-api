use base64::decode;
use std::{fs, path::PathBuf};

use chrono::{NaiveDate, Utc};
use openssl::{pkey::PKey, rsa::Rsa, sign::Verifier};
use ring::{
    agreement::EphemeralPrivateKey,
    signature::{
        UnparsedPublicKey, RSA_PKCS1_2048_8192_SHA256, RSA_PKCS1_2048_8192_SHA512,
        RSA_PKCS1_SHA256, RSA_PSS_2048_8192_SHA256, RSA_PSS_SHA256,
    },
};
use rocket::{http::uri::Path, serde::json};
use serde::{Deserialize, Serialize};

use crate::LICENSE_VALID;

const PUBLIC_KEY_PEM: &str = "
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAm9nO2zMvFc0XimeeB+5l
WPIyRCIzpxTQH5KF1nuC/iNcos93+TDYHC8S2mFx2dwcpc9Hd7IO0ryUvuwVm7EI
i1HdSjj7PnGW75svR8yqrdyleKVsE3kkyitfwijlRcfdB76imyxwpJYFfwGfSAxb
k3ZUDSIswNJrsyZL6dvPSzSJVi3JpQTyIPf34nMoY1+ebZ+X+tJ3EOGAdPaGWpEw
9+Hs7y7rV8lcM292L2+CX1msOCEva2iaXJdvx8Hw1BPhgQQwV+1W+bSddZZxwjw0
kT+tX9SG2x3MNxIf2W2vlEnaASgR+8ldLVNQNYPrhZ55oxDReEkKkjHAXuQpsfJb
vQIDAQAB
-----END PUBLIC KEY-----
";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct License {
    pub user_name: String,
    pub license_key: String,
    pub expiration_date: NaiveDate,
    pub mode: String,
    pub signature: String,
}

#[derive(Debug)]
pub struct LicenceService {
    public_key: PKey<openssl::pkey::Public>,
    license: Option<License>,
}

impl LicenceService {
    pub async fn new() -> Self {
        let public_key = PUBLIC_KEY_PEM.as_bytes().to_vec();
        let rsa = Rsa::public_key_from_pem(&public_key).expect("Unable to parse public key");
        let public_key = PKey::from_rsa(rsa).expect("Unable to create PKey from RSA");

        let license = match fs::File::open("license.json") {
            Err(_) => None,
            Ok(license) => match serde_json::from_reader(&license) {
                Err(_) => None,
                Ok(license) => Some(license),
            },
        };
        Self {
            public_key,
            license,
        }
    }

    pub fn is_valid(&self) -> Option<License> {
        match &self.license {
            None => None,
            Some(license) => {
                let signature =
                    base64::decode(&license.signature).expect("Failed to decode signature");
                let license_data = format!(
                    "{}.{}.{}.{}",
                    license.user_name, license.license_key, license.expiration_date, license.mode
                );

                let mut verifier = Verifier::new(openssl::hash::MessageDigest::sha256(), &self.public_key)
                    .expect("Unable to create verifier");
                verifier.update(license_data.as_bytes()).expect("Unable to update verifier");

                match verifier.verify(&signature).expect("Unable to verify signature") {
                    true => {}
                    false => return None
                };

                let today = Utc::now().naive_utc().date();
                if license.expiration_date <= today {
                    return None;
                }
                Some(license.to_owned())
            }
        }
    }
}
