use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultResponse<T> {
    pub message: String,
    pub data: T,
}

impl<T> DefaultResponse<T> {
    pub fn new(message: &str, data: T) -> DefaultResponse<T> {
        DefaultResponse {
            message: message.to_string(),
            data,
        }
    }
}
