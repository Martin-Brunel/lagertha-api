use std::env;

use aes_gcm::aead::OsRng;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand_core::RngCore;
use sha2::{Digest, Sha256};
use crate::services::hsm::HsmService;

pub struct Crypto;

impl Crypto {
    /// Generates a unique initialization vector (IV) for cryptographic operations.
    ///
    /// This function uses a cryptographically secure random number generator to produce
    /// a 128-bit (16 bytes) IV, which is suitable for many encryption algorithms including
    /// AES-128. The generated IV is then encoded as a hexadecimal string for ease of use
    /// and storage.
    ///
    /// # Returns
    ///
    /// Returns the generated IV as a hexadecimal string.
    pub fn generate_unique_iv() -> String {
        let mut rng = OsRng;
        match env::var("HSM_MODE").unwrap() == "1" {
            true => {
                let mut iv = [0u8; 16];
                rng.fill_bytes(&mut iv);
                hex::encode(iv)
            }
            false => {
                let mut iv = [0u8; 12];
                rng.fill_bytes(&mut iv);
                hex::encode(iv)
            }
        }
    }

    /// Generates a random AES-256 encryption key.
    ///
    /// This function uses a cryptographically secure random number generator
    /// to produce a 256-bit (32 bytes) key suitable for AES-256 encryption.
    ///
    /// # Returns
    ///
    /// Returns the generated key as a hexadecimal string.
    pub fn generate_aes_256_key() -> String {
        let mut rng = OsRng;
        let mut iv = [0u8; 32];
        rng.fill_bytes(&mut iv);
        hex::encode(iv)
    }

    /// Generates a random AES-128 encryption key.
    ///
    /// This function uses a cryptographically secure random number generator
    /// to produce a 128-bit (16 bytes) key suitable for AES-128 encryption.
    ///
    /// # Returns
    ///
    /// Returns the generated key as a hexadecimal string.
    pub fn generate_aes_128_key() -> String {
        let mut rng = OsRng;
        let mut iv = [0u8; 16];
        rng.fill_bytes(&mut iv);
        hex::encode(iv)
    }

    /// Calculates the SHA-256 hash of the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to the string representing the key for which the hash (checksum) is to be calculated.
    ///
    /// # Returns
    ///
    /// Returns the SHA-256 hash of the input key as a hexadecimal string.
    pub fn key_sum(key: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let sum = format!("{:x}", hasher.finalize());
        sum
    }

    /// Decrypts an encrypted string using an encryption tag and an initialization vector (IV).
    ///
    /// This function leverages an HSM (Hardware Security Module) service for decryption,
    /// utilizing an encryption tag specified in the environment variables.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - The encrypted string that needs to be decrypted.
    /// * `iv` - The initialization vector used during the encryption process.
    ///
    /// # Returns
    ///
    /// Returns the decrypted string.
    ///
    /// # Panics
    ///
    /// Panics if the environment variable `HSM_TAG` is not set.
    pub fn decrypt(encrypted: String, iv: String) -> String {
        match env::var("HSM_MODE").unwrap() == "1" {
            true => {
                let tag = env::var("HSM_TAG").expect("failed to get HSM_TAG key");
                HsmService::new().decrypt(&tag, iv, encrypted)
            }
            false => {
                let general_key = env::var("ENCRYPTION_KEY").unwrap();
                let to_decode = hex::decode(encrypted).unwrap();
                let byte_key = hex::decode(general_key).unwrap();
                let byte_nonce = hex::decode(iv).unwrap();
                let master_key = Key::<Aes256Gcm>::from_slice(byte_key.as_slice());
                let nonce = Nonce::from_slice(byte_nonce.as_slice());
                let cipher = Aes256Gcm::new(master_key);
                match cipher.decrypt(nonce, to_decode.as_ref()) {
                    Ok(plaintext) => String::from_utf8_lossy(&plaintext).to_string(),
                    Err(_) => String::from(""),
                }
            }
        }
    }

    /// Encrypts a string using an encryption tag and an initialization vector (IV).
    ///
    /// This function leverages an HSM (Hardware Security Module) service for encryption,
    /// utilizing an encryption tag specified in the environment variables.
    ///
    /// # Arguments
    ///
    /// * `key` - The string that needs to be encrypted.
    /// * `iv` - The initialization vector to be used for the encryption process.
    ///
    /// # Returns
    ///
    /// Returns the encrypted string.
    ///
    /// # Panics
    ///
    /// Panics if the environment variable `HSM_TAG` is not set.
    pub fn encrypt(key: String, iv: String) -> String {
        match env::var("HSM_MODE").unwrap() == "1" {
            true => {
                let tag = env::var("HSM_TAG").expect("failed to get HSM_TAG key");
                HsmService::new().encrypt(&tag, iv, key)
            }
            false => {
                let general_key = env::var("ENCRYPTION_KEY").unwrap();
                let byte_key = hex::decode(general_key).unwrap();
                let byte_nonce = hex::decode(iv).unwrap();
                let master_key = Key::<Aes256Gcm>::from_slice(byte_key.as_slice());
                let nonce = Nonce::from_slice(byte_nonce.as_slice());
                let cipher = Aes256Gcm::new(master_key);
                let ciphertext = cipher.encrypt(nonce, key.as_bytes()).unwrap();
                hex::encode(ciphertext)
            }
        }
    }
}
