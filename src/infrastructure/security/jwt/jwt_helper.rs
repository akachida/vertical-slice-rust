use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::{error, info};
use std::env;

use crate::{
    domain::user::user::User,
    infrastructure::security::{jwt::jwt_errors::JwtError, user_claims::UserClaims},
};

use super::refresh_token_claims::RefreshTokenClaims;

const ALGORITHM: Algorithm = Algorithm::HS512;

pub struct JwtHelper {}

impl JwtHelper {
    pub fn generate(user_data: &User) -> Result<String, JwtError> {
        let secret = env::var("AUTH_SECRET").unwrap_or_else(|_| "".to_string());
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(20))
            .unwrap()
            .timestamp();
        let claims = UserClaims {
            exp: expiration,
            sub: user_data.id().to_string(),
            data: user_data.to_owned(),
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

    pub fn generate_refresh_token(user_data: &User) -> Result<String, JwtError> {
        let refresh_secret = env::var("REFRESH_TOKEN_SECRET").unwrap_or_else(|_| "".to_string());
        let refresh_expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .unwrap()
            .timestamp();
        let header = Header::new(ALGORITHM);

        encode(
            &header,
            &RefreshTokenClaims {
                exp: refresh_expiration,
                sub: user_data.id().to_string(),
            },
            &EncodingKey::from_secret(refresh_secret.as_bytes()),
        )
        .map_err(|_| {
            error!(
                "Error while trying to generate a JWT Token {}",
                user_data.id().to_string()
            );
            JwtError::TokenCreationError
        })
    }

    pub fn decode_token(token: String) -> Result<UserClaims, JwtError> {
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
            let decode_err = decode.unwrap_err().to_string();

            info!(
                "Attempt of using an invalid token: {}",
                decode_err.to_string()
            );

            return Err(JwtError::InvalidToken);
        }

        let user_claims = decode.unwrap().claims;

        if user_claims.exp < Utc::now().timestamp() {
            info!("Token expired for {}", &user_claims.sub);

            return Err(JwtError::TokenExpired);
        }

        Ok(user_claims)
    }

    pub fn decode_refresh_token(token: String) -> Result<RefreshTokenClaims, JwtError> {
        if token.is_empty() {
            info!("Refresh token was empty");
            return Err(JwtError::EmptyToken);
        }

        let secret = env::var("REFRESH_TOKEN_SECRET").unwrap_or_else(|_| "".to_string());
        let decode = decode::<RefreshTokenClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS512),
        );

        if decode.is_err() {
            let decode_err = decode.unwrap_err().to_string();

            info!(
                "Attempt of using an invalid refresh token: {}",
                decode_err.to_string()
            );

            return Err(JwtError::InvalidToken);
        }

        let refresh_token_claims = decode.unwrap().claims;

        if refresh_token_claims.exp < Utc::now().timestamp() {
            info!("Refresh token expired for {}", &refresh_token_claims.sub);

            return Err(JwtError::TokenExpired);
        }

        Ok(refresh_token_claims)
    }
}
