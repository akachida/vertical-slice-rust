use std::env;

use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use sha3::{Digest, Sha3_256};

use crate::domain::value_objects::hashed_password::{HashedPassword, HashedPasswordError};

#[test]
fn create_new_hashed_password_successfully() {
    // arrange
    dotenv::dotenv().ok();
    let password = "112233";
    let salt = format!("{:x}", Sha3_256::digest(password));
    let secret = env::var("PASSWORD_SECRET").unwrap();
    let hasher = Argon2::new_with_secret(
        secret.as_bytes(),
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(65536, 2, 1, None).unwrap(),
    )
    .unwrap();
    let password_hash = hasher.hash_password(password.as_bytes(), &salt);
    let phc_string = password_hash.unwrap().to_string();

    // act
    let sut = HashedPassword::new(password).unwrap();

    // assert
    assert_eq!(sut.to_string(), phc_string);
}

#[test]
fn create_new_hashed_password_from_hash_successfully() {
    // arrange
    dotenv::dotenv().ok();
    let hashed_password = HashedPassword::new("123123").unwrap();

    // act
    let from_hash = HashedPassword::new_from_hash(&hashed_password.to_string()).unwrap();

    // assert
    assert_eq!(hashed_password.to_string(), from_hash.to_string());
}

#[test]
fn fails_when_create_hashed_password_from_invalid_hash() {
    // arrange
    dotenv::dotenv().ok();
    let hashed_password = "invalid_hash_password";

    // act
    let from_hash = HashedPassword::new_from_hash(hashed_password).unwrap_err();

    // assert
    assert_eq!(from_hash, HashedPasswordError::InvalidPasswordHash);
}

#[test]
fn fail_hashing_when_password_is_empty() {
    // arrange
    let password = "";

    // act
    let sut = HashedPassword::new(password).unwrap_err();

    // assert
    assert_eq!(sut, HashedPasswordError::EmptyPassword);
}

#[test]
fn return_true_when_verify_password() {
    // arrange
    dotenv::dotenv().ok();
    let password = "112233";
    let hashed_password = HashedPassword::new(password).unwrap();

    // act
    let sut = hashed_password.verify("112233");

    // assert
    assert!(sut);
}

#[test]
fn return_false_when_password_dont_match_stored_hash() {
    // arrange
    dotenv::dotenv().ok();
    let password = "112233";
    let hashed_password = HashedPassword::new(password).unwrap();

    // act
    let sut = hashed_password.verify("aabbcc");

    // assert
    assert!(!sut);
}
