use serde::{Serialize, Deserialize};
use crate::business::security;
use super::UserRole;

#[derive(Serialize, Deserialize)]
struct UserAuthClaims {
    id: i32,
    role: UserRole,
    exp: usize,
}

pub fn token_from_claims(id: i32, role: UserRole, expiration_sec: i64) -> Result<String, ()> {
    security::jwt_create(&mut UserAuthClaims { id, role, exp: 0 }, expiration_sec)
}

pub fn claims_from_token(token: &String) -> Result<(i32, UserRole), ()> {
    security::jwt_decode::<UserAuthClaims>(token)
        .map(|claims| (claims.id, claims.role))
}

impl security::JwtClaims for UserAuthClaims {
    fn set_expiration(&mut self, exp: usize) {
        self.exp = exp;
    }
}