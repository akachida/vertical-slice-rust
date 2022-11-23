use crate::{
    application::user::queries::get_user_query::GetUserQuery,
    infrastructure::{
        application_error_response::ApplicationErrorResponse,
        rest::request_validation::RequestValidation,
    },
};

impl RequestValidation for GetUserQuery {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        if self.id.is_nil() {
            return Err(ApplicationErrorResponse {
                message: "Error while validating GetUser request".to_string(),
                error_code: 400,
                details: vec!["ID should not be empty".to_string()],
                inner: None,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::{
        application::user::queries::get_user_query::GetUserQuery,
        infrastructure::rest::request_validation::RequestValidation,
    };

    #[test]
    pub fn validation_error_if_id_is_nil() {
        // arrange
        let query = GetUserQuery { id: Uuid::nil() };

        // act
        let sut = query.validate().unwrap_err();

        // assert
        assert_eq!(
            sut.message,
            "Error while validating GetUser request".to_string()
        );
        assert_eq!(sut.error_code, 400);
        assert_eq!(sut.details, vec!["ID should not be empty".to_string()]);
        assert!(sut.inner.is_none())
    }
}
