use crate::{
    application::auth::query::auth_login_query::AuthLoginQuery,
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for AuthLoginQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        let mut error_messages: Vec<String> = Vec::new();

        if self.username.is_empty() {
            error_messages.append(&mut vec!["Username is required".to_string()]);
        }

        if self.password.is_empty() {
            error_messages.append(&mut vec!["Password is required".to_string()]);
        }

        if !error_messages.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Username or password was not valid".to_string(),
                error_code: 400,
                details: error_messages,
                inner: None,
            });
        }

        Ok(())
    }
}
