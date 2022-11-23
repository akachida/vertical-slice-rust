use crate::{
    application::auth::query::refresh_token_query::RefreshTokenQuery,
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for RefreshTokenQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        if self.refresh_token.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Refresh token is empty".to_string(),
                error_code: 400,
                details: vec![],
                inner: None,
            });
        }

        Ok(())
    }
}
