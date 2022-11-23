use actix_web::{http::StatusCode, HttpResponse, Result};
use migration::DbErr;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApplicationErrorResponse {
    pub message: String,
    pub error_code: u16,
    pub details: Vec<String>,
    pub inner: Option<Box<ApplicationErrorResponse>>,
}

pub trait ApplicationErrorResponseTrait {
    fn into_http_response(&self) -> Result<HttpResponse>;
}

impl ApplicationErrorResponseTrait for ApplicationErrorResponse {
    fn into_http_response(&self) -> Result<HttpResponse> {
        let error_response =
            serde_json::to_string(&self).map_err(|_| "No error message".to_string());

        let status_code = match StatusCode::from_u16(self.error_code) {
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Ok(status_code) => status_code,
        };

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

pub trait DefaultApplicationErrorResponseTrait {
    fn into_application_error_response(&self) -> ApplicationErrorResponse;
}

impl DefaultApplicationErrorResponseTrait for DbErr {
    fn into_application_error_response(&self) -> ApplicationErrorResponse {
        log::error!("Error while doing a data request: {}", self.to_string());

        ApplicationErrorResponse {
            message: self.to_string(),
            error_code: 500,
            details: vec![],
            inner: None,
        }
    }
}
