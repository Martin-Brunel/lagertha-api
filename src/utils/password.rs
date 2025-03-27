use rand::Rng;

pub struct PasswordUtils;

impl PasswordUtils {
    pub fn hash_password(new_password: String) -> String {
        let encoded = bcrypt::hash(new_password, 12).expect("fail to hash");
        encoded
    }
    
    pub fn compare_hash(password: String, hash: String) -> bool {
        bcrypt::verify(password, &hash).unwrap()
    }

    pub fn generate_password(length: usize) -> String {
        let charset =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

        let mut rng = rand::thread_rng();

        let password: String = (0..length)
            .map(|_| {
                let random_index = rng.gen_range(0..charset.len());
                charset.chars().nth(random_index).unwrap()
            })
            .collect();

        password
    }

    pub fn verify_password(password: &str) -> bool {
        if password.len() < 8 {
            return false;
        }
        if !password.chars().any(char::is_uppercase) {
            return false;
        }
        if !password.chars().any(char::is_lowercase) {
            return false;
        }
        if !password.chars().any(char::is_numeric) {
            return false;
        }
        true
    }
}
