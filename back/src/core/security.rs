use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Serialize, de::DeserializeOwned};
use std::env;

pub fn password_salt_and_hash(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
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

pub fn fake_password_verify() {
    password_verify(
        &"iadAIDAG~~~ZI##123611#{{__".to_string(),
        &"$argon2id$v=19$m=4096,t=3,p=1$KaoNKL8Ce5qieyRqDMbaXg$O2z+Xx2GUZOMmf2zFtyt7nu9xGm8nvfKSKS7bxTN9wg".to_string()
    );
}

pub trait JwtClaims {
    fn set_expiration(&mut self, exp: usize);
}

pub fn jwt_decode<C: JwtClaims + DeserializeOwned>(jwt: &String) -> Result<C, ()> {
    jsonwebtoken::decode::<C>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(
            env::var("JWT_SECRET_KEY")
                .expect("JWT_SECRET_KEY not set")
                .as_bytes()
        ),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512)
    )
    .map(|decoded| decoded.claims)
    .map_err(|_| ())
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
