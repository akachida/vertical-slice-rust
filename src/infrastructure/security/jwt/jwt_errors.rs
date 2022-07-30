use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Token was empty")]
    EmptyToken,
    #[error("Invalid Token")]
    InvalidToken,
    #[error("Token creation error")]
    TokenCreationError,
}
