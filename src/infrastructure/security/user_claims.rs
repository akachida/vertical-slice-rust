use serde::{Deserialize, Serialize};

use crate::domain::user::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub exp: i64,
    pub sub: String,
    pub data: User,
}
