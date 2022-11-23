use crate::infrastructure::application_error_response::ApplicationErrorResponse;

// TIP: for future changes, probably should change this to async
pub trait RequestValidation {
    fn validate(&self) -> Result<(), ApplicationErrorResponse>;
}
