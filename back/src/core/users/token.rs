use serde::{Serialize, Deserialize};
use crate::core::security;
use super::UserRole;

pub struct Claims {
    pub id: i32,
    pub role: UserRole
}

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    id: i32,
    role: UserRole,
    exp: usize,
}

pub fn from_claims(claims: Claims, expiration_sec: i64) -> Result<String, ()> {
    security::jwt_create(&mut JwtClaims { id: claims.id, role: claims.role, exp: 0 }, expiration_sec)
}

pub fn to_claims(token: &String) -> Result<Claims, ()> {
    security::jwt_decode::<JwtClaims>(token)
        .map(|claims| Claims { id: claims.id, role: claims.role })
}

impl security::JwtClaims for JwtClaims {
    fn set_expiration(&mut self, exp: usize) {
        self.exp = exp;
    }
}