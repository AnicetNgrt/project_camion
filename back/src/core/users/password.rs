use serde::Serialize;
use crate::core::security;

#[derive(Serialize)]
pub enum Weakness {
    NotLongEnough,
    NoUpperCase,
    NoLowerCase,
    NoSpecialChars,
    NoNumeric,
    NoAlphabetic,
}

pub fn find_weaknesses(password: &String) -> Option<Vec<Weakness>> {
    let mut weaknesses = Vec::<Weakness>::new();

    if password.len() < 8 {
        weaknesses.push(Weakness::NotLongEnough);
    }

    if !password.chars().any(|c| c.is_digit(10)) {
        weaknesses.push(Weakness::NoNumeric);
    }
    if !password.chars().any(char::is_alphabetic) {
        weaknesses.push(Weakness::NoAlphabetic);
    }
    if password.chars().all(char::is_alphanumeric) {
        weaknesses.push(Weakness::NoSpecialChars);
    }

    if !password.chars().any(char::is_uppercase) {
        weaknesses.push(Weakness::NoUpperCase);
    }
    if !password.chars().any(char::is_lowercase) {
        weaknesses.push(Weakness::NoLowerCase);
    }

    if weaknesses.len() > 0 {
        Some(weaknesses)
    } else {
        None
    }
}

pub fn hash(password: &String) -> Result<String, ()> {
    security::password_salt_and_hash(password)
        .map_err(|_| ())
}