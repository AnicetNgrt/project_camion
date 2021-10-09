use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub enum PasswordWeakness {
    NotLongEnough,
    NoUpperCase,
    NoLowerCase,
    NoSpecialChars,
    NoNumeric,
    NoAlphabetic,
}

pub fn password_find_weaknesses(password: &String) -> Option<Vec<PasswordWeakness>> {
    let mut weaknesses = Vec::<PasswordWeakness>::new();

    if password.len() < 8 {
        weaknesses.push(PasswordWeakness::NotLongEnough);
    }

    if !password.chars().any(|c| c.is_digit(10)) {
        weaknesses.push(PasswordWeakness::NoNumeric);
    }
    if !password.chars().any(char::is_alphabetic) {
        weaknesses.push(PasswordWeakness::NoAlphabetic);
    }
    if password.chars().all(char::is_alphanumeric) {
        weaknesses.push(PasswordWeakness::NoSpecialChars);
    }

    if password.chars().all(char::is_lowercase) {
        weaknesses.push(PasswordWeakness::NoUpperCase);
    } else if password.chars().all(char::is_uppercase) {
        weaknesses.push(PasswordWeakness::NoLowerCase);
    }

    if weaknesses.len() > 0 {
        Some(weaknesses)
    } else {
        None
    }
}

pub fn password_salt_and_hash(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password_simple(password.as_bytes(), &salt)?
        .to_string())
}

pub fn password_verify(password: &String, hash: &String) -> bool {
    let argon2 = Argon2::default();
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        _ => false,
    }
}

pub trait JwtClaims {
    fn set_expiration(&mut self, exp: usize);
}

pub fn jwt_create<C: JwtClaims + Serialize>(
    base_claims: &mut C,
    expiration_sec: i64,
) -> Result<String, ()> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration_sec))
        .expect("valid timestamp")
        .timestamp();
    base_claims.set_expiration(expiration as usize);
    let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
    jsonwebtoken::encode(
        &header,
        &base_claims,
        &jsonwebtoken::EncodingKey::from_secret(
            env::var("JWT_SECRET_KEY")
                .expect("JWT_SECRET_KEY not set")
                .as_bytes(),
        ),
    )
    .map_err(|_| ())
}
