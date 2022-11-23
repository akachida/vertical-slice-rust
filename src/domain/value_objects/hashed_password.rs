use core::fmt;
use std::env;

use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct HashedPassword {
    value: String,
}

impl HashedPassword {
    pub fn new(value: &str) -> Result<Self, HashedPasswordError> {
        if let Err(validation_error) = Self::validate(value) {
            return Err(validation_error);
        };

        let salt = format!("{:x}", Sha3_256::digest(value));
        let secret = env::var("PASSWORD_SECRET").unwrap();

        let settings = HashedPassword::hash_configuration(&secret);

        if let Err(hasher_error) = settings {
            return Err(hasher_error);
        };

        let password_hash = settings
            .unwrap()
            .hash_password(value.as_bytes(), &salt)
            .map_err(|_| HashedPasswordError::HashingPassword);

        if let Err(password_hash_error) = password_hash {
            return Err(password_hash_error);
        };

        Ok(HashedPassword {
            value: password_hash.unwrap().to_string(),
        })
    }

    pub fn validate(value: &str) -> Result<bool, HashedPasswordError> {
        if value.is_empty() {
            return Err(HashedPasswordError::EmptyPassword);
        }

        if value.len() < 8 {
            return Err(HashedPasswordError::MinimumCharacters);
        }

        let required_characters_regex = RegexSet::new(&[
            r"(.*[a-z]){2,}", // at least 2 uppercase characters
            r"(.*[A-Z]){2,}", // at lease 2 lowercase characters
            r"(.*[0-9]){2,}", // at least 2 numeric characters
            r"(.*[!@#$&*])+", // at least 1 special characters between ! @ # $ & and *
        ])
        .unwrap();
        let regex_matches: Vec<_> = required_characters_regex
            .matches(&value)
            .into_iter()
            .collect();

        if regex_matches.len() != 4 {
            return Err(HashedPasswordError::RequiredCharacters);
        }

        Ok(true)
    }

    pub fn new_from_hash(value: &str) -> Result<Self, HashedPasswordError> {
        let hasher = PasswordHash::try_from(value);

        if hasher.is_err() {
            return Err(HashedPasswordError::InvalidPasswordHash);
        }

        Ok(Self {
            value: value.to_string(),
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
    #[error("Password doesn't match the required characters")]
    RequiredCharacters,
    #[error("Invalid password hash")]
    InvalidPasswordHash,
    #[error("Couldn't generate hasher for password")]
    GeneratinArgon2Hasher,
    #[error("Error while hashing the password")]
    HashingPassword,
}
