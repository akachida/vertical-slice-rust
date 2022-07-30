use actix_web::{http::StatusCode, HttpResponse, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApplicationErrorResponse {
    pub message: String,
    pub error_code: u16,
    pub details: Vec<String>,
    pub inner: Option<Box<ApplicationErrorResponse>>,
}

impl ApplicationErrorResponse {
    pub fn into_http_response(&self) -> Result<HttpResponse> {
        let error_response =
            serde_json::to_string(&self).map_err(|_| "No error message".to_string());
        // TODO: map error to 500 when not set or unwrap fails
        let status_code = StatusCode::from_u16(self.error_code).unwrap();

        Ok(HttpResponse::build(status_code).body(error_response.unwrap()))
    }
}

impl Default for ApplicationErrorResponse {
    fn default() -> Self {
        Self {
            message: "Something bad happened".to_string(),
            error_code: 500,
            details: vec![],
            inner: None,
        }
    }
}
