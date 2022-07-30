use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    Writer,
    Reader,
}

impl Role {
    pub fn from_i16(v: i16) -> Self {
        match v {
            1 => Role::Admin,
            2 => Role::Writer,
            _ => Role::Writer,
        }
    }
}
