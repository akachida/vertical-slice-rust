use core::fmt;

use email_address_parser::EmailAddress;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(v: &str) -> Result<Self, EmailError> {
        if v.is_empty() {
            return Err(EmailError::EmailValueEmpty);
        }

        if !Self::validate(v) {
            return Err(EmailError::InvalidEmailFormat);
        }

        Ok(Email {
            value: v.to_string(),
        })
    }

    pub fn validate(v: &str) -> bool {
        EmailAddress::parse(v, None).is_some()
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum EmailError {
    #[error("Email value is empty")]
    EmailValueEmpty,
    #[error("Invalid email format")]
    InvalidEmailFormat,
}
