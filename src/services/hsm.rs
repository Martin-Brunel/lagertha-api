use cryptoki::object::{Attribute, KeyType, ObjectClass};
use cryptoki::session::{Session, UserType};
use cryptoki::slot::Slot;
use cryptoki::types::AuthPin;
use cryptoki::{
    context::{CInitializeArgs, Pkcs11},
    mechanism::Mechanism,
};
use dotenv::dotenv;
use std::env;

use crate::utils::cli::CLIUtils;

/// ### HsmService
///
/// provide methods to interact with hsm throw pkcs11 file
pub struct HsmService {
    pkcs11: Pkcs11,
}

impl HsmService {
    /// Link Hsm throw PKCS11 and init the service
    pub fn new() -> Self {
        dotenv().ok();
        let pcks11: Pkcs11 = Pkcs11::new(
            env::var("HSM_SO_PATH")
                .unwrap_or_else(|_| "/usr/local/lib/softhsm/libsofthsm2.so".to_string()),
        )
        .unwrap();
        HsmService { pkcs11: pcks11 }
    }

    /// Init HSM for the first use
    /// 
    /// link the slot and provide the SO pin
    pub fn init(&self) -> Result<bool, String> {
        self.pkcs11.initialize(CInitializeArgs::OsThreads).unwrap();

        let slots = self.pkcs11.get_all_slots().unwrap();
        let mut slot_to_init = None;

        for slot in slots {
            if slot_to_init.is_none() {
                let token_info = self.pkcs11.get_token_info(slot).unwrap();
                match token_info.token_initialized() {
                    true => {},
                    false => slot_to_init = Some(slot),
                };
            }
        }
        match slot_to_init {
            None => Err(String::from("No slot to init")),
            Some(slot) => {
                let pin = CLIUtils::prompt("enter your SO_PIN:");
                let so_pin = AuthPin::new(pin);
                let token_label = env::var("HSM_TOKEN_LABEL").unwrap();
                self.pkcs11.init_token(slot, &so_pin, &token_label).unwrap();
                let user_pin = env::var("HSM_USER_PIN").unwrap();
                {
                    let session = self.pkcs11.open_rw_session(slot).unwrap();
                    session.login(UserType::So, Some(&so_pin)).unwrap();
                    session
                        .init_pin(&AuthPin::new(user_pin.clone().into()))
                        .unwrap();
                }
                CLIUtils::empty_line();
                CLIUtils::separator();
                CLIUtils::empty_line();
                CLIUtils::write("HSM init");
                CLIUtils::separator();
                CLIUtils::empty_line();
                Ok(true)
            }
        }
    }

    pub fn get_slot(&self) -> Result<Slot, String> {
        self.pkcs11
            .initialize(CInitializeArgs::OsThreads)
            .expect("fail to init");
        let slots = self
            .pkcs11
            .get_slots_with_token()
            .expect("fail to get slot with token");
        let label = env::var("HSM_TOKEN_LABEL").unwrap();
        for slot in slots {
            let token = self
                .pkcs11
                .get_token_info(slot)
                .expect("fail to get token info");
            let token_label = token.label();
            if token_label == &label {
                return Ok(slot);
            }
        }
        Err(String::from("not found"))
    }

    pub fn connect(&self) -> Session {
        let slot = self.get_slot().unwrap();
        let session = self.pkcs11.open_rw_session(slot).unwrap();
        let pin = env::var("HSM_USER_PIN").unwrap();
        let user_pin = AuthPin::new(pin.into());
        session
            .login(UserType::User, Some(&user_pin))
            .expect("fail to log");
        session
    }

    pub fn encrypt(&self, tag: &str, iv: String, plain: String) -> String {
        let session = self.connect();
        let master_key_label = Attribute::Label(tag.as_bytes().to_vec());
        let key_object = match session.find_objects(&[master_key_label.clone()]) {
            Ok(objects) => {
                if objects.len() > 0 {
                    objects.clone().remove(0)
                } else {
                    let key_template = vec![
                        Attribute::Class(ObjectClass::SECRET_KEY),
                        Attribute::KeyType(KeyType::AES),
                        Attribute::Token(true),
                        Attribute::Sensitive(true),
                        Attribute::Private(true),
                        Attribute::ValueLen(32.into()),
                        Attribute::Derive(true),
                    ];
                    let mut master_key_template = key_template.clone();
                    master_key_template.insert(0, master_key_label.clone());
                    session
                        .generate_key(&Mechanism::AesKeyGen, &master_key_template)
                        .unwrap()
                }
            }
            Err(_) => {
                let key_template = vec![
                    Attribute::Class(ObjectClass::SECRET_KEY),
                    Attribute::KeyType(KeyType::AES),
                    Attribute::Token(true),
                    Attribute::Sensitive(true),
                    Attribute::Private(true),
                    Attribute::ValueLen(32.into()),
                    Attribute::Derive(true),
                ];
                let mut master_key_template = key_template.clone();
                master_key_template.insert(0, master_key_label.clone());
                session
                    .generate_key(&Mechanism::AesKeyGen, &master_key_template)
                    .unwrap()
            }
        };
        let iv = hex::decode(iv).unwrap();
        let mut array = [0u8; 16];
        array.copy_from_slice(&iv);
        let mechanism = Mechanism::AesCbc(array);
        let cipher = session
            .encrypt(&mechanism, key_object, &plain.into_bytes())
            .unwrap();
        hex::encode(cipher)
    }

    pub fn decrypt(&self, tag: &str, iv: String, encrypted_data: String) -> String {
        let session = self.connect();
        let encrypted_data = hex::decode(encrypted_data).unwrap();
        let master_key_label = Attribute::Label(tag.as_bytes().to_vec());
        let key_object = session
            .find_objects(&[master_key_label.clone()])
            .unwrap()
            .clone()
            .remove(0);
        let iv = hex::decode(iv).unwrap();
        let mut array = [0u8; 16];
        array.copy_from_slice(&iv);
        let mechanism = Mechanism::AesCbc(array);
        let decrypted = session
            .decrypt(&mechanism, key_object, encrypted_data.as_slice())
            .unwrap();
        String::from_utf8_lossy(&decrypted).to_string()
    }
}
