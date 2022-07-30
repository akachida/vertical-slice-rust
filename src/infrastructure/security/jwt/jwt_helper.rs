use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use log::{error, info};
use std::env;

use crate::{
    domain::user::user::User,
    infrastructure::security::{jwt::jwt_errors::JwtError, user_claims::UserClaims},
};

const ALGORITHM: Algorithm = Algorithm::HS512;

pub struct JwtHelper {}

impl JwtHelper {
    pub fn generate(user_data: &User) -> Result<String, JwtError> {
        let secret = env::var("AUTH_SECRET").unwrap_or_else(|_| "".to_string());
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(1))
            .expect("Valida timestamp")
            .timestamp();
        let claims = UserClaims {
            exp: expiration as usize,
            sub: user_data.id().to_string(),
            data: serde_json::to_string(&user_data).unwrap_or_else(|_| "".to_string()),
        };
        let header = Header::new(ALGORITHM);

        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|_| {
            error!(
                "Error while trying to generate a JWT Token {}",
                user_data.id().to_string()
            );
            JwtError::TokenCreationError
        })
    }

    pub fn decode_token(token: String) -> Result<TokenData<UserClaims>, JwtError> {
        if token.is_empty() {
            info!("Token was empty");
            return Err(JwtError::EmptyToken);
        }

        let secret = env::var("AUTH_SECRET").unwrap_or_else(|_| "".to_string());
        let decode = decode::<UserClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        if decode.is_err() {
            info!("Attempt of using an invalid token");
            return Err(JwtError::InvalidToken);
        }

        Ok(decode.unwrap())
    }
}
