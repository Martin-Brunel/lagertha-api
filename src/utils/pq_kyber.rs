use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use pqc_kyber::*;

use ml_kem::KemCore;
use ml_kem::{self, EncodedSizeUser};
pub use pqcrypto_kyber::kyber512;
use pqcrypto_kyber::kyber512::{PublicKey, SecretKey};
use rand_core;

use super::crypto::Crypto;
pub struct PQKyber;

impl PQKyber {
    pub fn generate_key_pair() -> ([u8; KYBER_PUBLICKEYBYTES], [u8; KYBER_SECRETKEYBYTES]) {
        let mut rng = rand::thread_rng();
        let key_pair = keypair(&mut rng).unwrap();
        (key_pair.public, key_pair.secret)
    }

    pub fn generate_key_pair_512() -> ([u8; 800], [u8; 1632]) {
        let mut rng = rand::thread_rng();

        // Generate a (decapsulation key, encapsulation key) pair
        let (sk, pk) = ml_kem::MlKem512::generate(&mut rng);
        // Convert the keys to byte arrays
        let sk_bytes = {
            let bytes: &[u8] = &sk.as_bytes();
            let mut array = [0u8; 1632];
            array.copy_from_slice(bytes);
            array
        };

        let pk_bytes = {
            let bytes: &[u8] = &pk.as_bytes();
            let mut array = [0u8; 800];
            array.copy_from_slice(bytes);
            array
        };

        (pk_bytes, sk_bytes)
    }

    pub fn encrypt_key(key: String, iv: String) -> String {
        Crypto::encrypt(key, iv)
    }

    pub fn decrypt_key(key: String, iv: String) -> String {
        Crypto::decrypt(key, iv)
    }

    // pub fn generate_aes_key_from_cipher(
    //     ciphertext: String,
    //     kyber_sk: String,
    //     iv: String,
    // ) -> String {
    //     let cipher = hex::decode(ciphertext).unwrap();
    //     let sk = hex::decode(Self::decrypt_key(kyber_sk, iv)).unwrap();
    //     let aes_key = decapsulate(cipher.as_slice(), sk.as_slice()).expect("decapsulate error");
    //     hex::encode(aes_key)
    // }
}
