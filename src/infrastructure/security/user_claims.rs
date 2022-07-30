use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub exp: usize,
    pub sub: String,
    // TODO: change data to User model
    pub data: String,
}
