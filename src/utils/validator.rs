use regex::Regex;
use serde::{de::Error, Deserialize, Deserializer};

/// ### Email validator
///
/// validate the string to be an email string
pub fn email<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: String = Deserialize::deserialize(deserializer)?;

    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if email_regex.is_match(&raw_value) {
        Ok(raw_value)
    } else {
        Err(Error::custom("Invalid email"))
    }
}

/// ### Option Email validator
///
/// validate the string to be an email string
pub fn option_email<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: Option<String> = Deserialize::deserialize(deserializer)?;

    match raw_value {
        None => Ok(None),
        Some(raw_value) => {
            let email_regex =
                Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

            if email_regex.is_match(&raw_value) {
                Ok(Some(raw_value))
            } else {
                Err(Error::custom("Invalid email"))
            }
        }
    }
}

/// ### Password validator
///
/// check if a password string is strong
/// - min length 8
/// - at least 1 lowercase
/// - at least 1 uppercase
/// - at least 1 number
pub fn option_password<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: Option<String> = Deserialize::deserialize(deserializer).unwrap();
    match raw_value {
        None => Ok(None),
        Some(raw_value) => {
            if raw_value.len() < 8 {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_uppercase) {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_lowercase) {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_numeric) {
                return Err(Error::custom("Invalid password"));
            }
            Ok(Some(raw_value))
        }
    }
}

/// ### Password validator
///
/// check if a password string is strong
/// - min length 8
/// - at least 1 lowercase
/// - at least 1 uppercase
/// - at least 1 number
pub fn password<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: String = Deserialize::deserialize(deserializer).unwrap();
            if raw_value.len() < 8 {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_uppercase) {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_lowercase) {
                return Err(Error::custom("Invalid password"));
            }
            if !raw_value.chars().any(char::is_numeric) {
                return Err(Error::custom("Invalid password"));
            }
            Ok(raw_value)
}

/// ### vec Ip validator
///
/// check if an vec of string is an vec of IP string
pub fn vec_ip<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: Vec<String> = Deserialize::deserialize(deserializer).unwrap();
    let ip_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    for ip in &raw_value {
        if !ip_regex.is_match(&ip) {
            return Err(Error::custom("Invalid ip"));
        }
    }

    Ok(raw_value)
}

/// ### Ip validator
///
/// check if an string is an IP string
pub fn ip<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: String = Deserialize::deserialize(deserializer).unwrap();
    let ip_regex = Regex::new(r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$").unwrap();
    if !ip_regex.is_match(&raw_value) {
        return Err(Error::custom("Invalid ip"));
    }
    Ok(raw_value)
}
