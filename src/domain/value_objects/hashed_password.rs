use core::fmt;
use std::env;

use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HashedPassword {
    value: String,
}

impl HashedPassword {
    pub fn new(v: &str) -> Result<Self, HashedPasswordError> {
        if v.is_empty() {
            return Err(HashedPasswordError::EmptyPassword);
        }

        // TODO: add validations for minimum-number, case-sensitive and type of characters

        let salt = format!("{:x}", Sha3_256::digest(v));
        let secret = env::var("PASSWORD_SECRET").unwrap();

        let settings = HashedPassword::hash_configuration(&secret);

        if let Err(hasher_error) = settings {
            return Err(hasher_error);
        };

        let password_hash = settings
            .unwrap()
            .hash_password(v.as_bytes(), &salt)
            .map_err(|_| HashedPasswordError::HashingPassword);

        if let Err(password_hash_error) = password_hash {
            return Err(password_hash_error);
        };

        Ok(HashedPassword {
            value: password_hash.unwrap().to_string(),
        })
    }

    pub fn new_from_hash(v: &str) -> Result<Self, HashedPasswordError> {
        let hasher = PasswordHash::try_from(v);

        if hasher.is_err() {
            return Err(HashedPasswordError::InvalidPasswordHash);
        }

        Ok(Self {
            value: v.to_string(),
        })
    }

    pub fn hash_configuration(secret: &String) -> Result<Argon2<'_>, HashedPasswordError> {
        Argon2::new_with_secret(
            secret.as_bytes(),
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(65536, 2, 1, None).unwrap(),
        )
        .map_err(|_| HashedPasswordError::GeneratinArgon2Hasher)
    }

    pub fn verify(&self, password: &str) -> bool {
        let secret = env::var("PASSWORD_SECRET").unwrap();
        let settings = HashedPassword::hash_configuration(&secret).unwrap();
        let hasher = PasswordHash::try_from(self.value.as_str()).unwrap();
        let verify = settings.verify_password(password.as_bytes(), &hasher);

        verify.is_ok()
    }
}

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum HashedPasswordError {
    #[error("Empty password")]
    EmptyPassword,
    #[error("Password doesn't have the minimum characters")]
    MinimumCharacters,
    #[error("Password don't match the required characters")]
    RequiredCharacters,
    #[error("Invalid password hash")]
    InvalidPasswordHash,
    #[error("Couldn't generate hasher for password")]
    GeneratinArgon2Hasher,
    #[error("Error while hashing the password")]
    HashingPassword,
}
