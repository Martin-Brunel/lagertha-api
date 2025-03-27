extern crate std;
use base32::Alphabet;
use otp_rs::TOTP;
use rand::Rng;
extern crate chrono;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn _generate_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..7)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn generate_reset_code() -> String {
    const CHARSET: &[u8] = b"0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn generate_base32_key(length_in_bits: usize) -> String {
    let length_in_bytes = (length_in_bits + 7) / 8;
    let length_in_base32_chars = (length_in_bits + 4) / 5;
    let base32_alphabet = Alphabet::Rfc4648 { padding: false };
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..length_in_bytes).map(|_| rng.gen()).collect();
    let base32_key = base32::encode(base32_alphabet, &random_bytes);
    let truncated_base32_key = &base32_key[..length_in_base32_chars];
    truncated_base32_key.to_string()
}

pub fn check_otp(input: u32, secret_key: String) -> bool {
    let otp = TOTP::from_base32(&secret_key).unwrap();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let code = otp.generate(30, timestamp).unwrap();
    input == code
}
