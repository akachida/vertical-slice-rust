use crate::{
    application::user::commands::create_user_command::CreateUserCommand,
    domain::value_objects::{email::Email, hashed_password::HashedPassword},
    infrastructure::application_error_response::ApplicationErrorResponse,
    infrastructure::rest::request_validation::RequestValidation,
};

impl RequestValidation for CreateUserCommand {
    fn validate(&self) -> Result<(), ApplicationErrorResponse> {
        let mut error_messages: Vec<String> = Vec::new();

        if self.first_name.is_empty() {
            error_messages.append(&mut vec!["First name is empty".to_string()]);
        }

        if self.last_name.is_empty() {
            error_messages.append(&mut vec!["Last name is empty".to_string()]);
        }

        if self.email.is_empty() {
            error_messages.append(&mut vec!["Email is empty".to_string()]);
        }

        if self.role_id <= 0 {
            error_messages.append(&mut vec!["Role ID should be greater than 0".to_string()]);
        }

        if self.first_name.len() < 3 || self.first_name.len() > 30 {
            error_messages.append(&mut vec![
                "First name should have between 3 and 30 characters".to_string(),
            ]);
        }

        if self.last_name.len() < 3 || self.last_name.len() > 30 {
            error_messages.append(&mut vec![
                "Last name should have between 3 and 30 characters".to_string(),
            ]);
        }

        let valid_password = HashedPassword::validate(&self.password);

        if valid_password.is_err() {
            error_messages.append(&mut vec![valid_password.unwrap_err().to_string()]);
        }

        if !Email::validate(&self.email) {
            error_messages.append(&mut vec!["Email it's not a valid email address".to_string()]);
        }

        if !error_messages.is_empty() {
            return Err(ApplicationErrorResponse {
                message: "Error while trying to create a new User".to_string(),
                error_code: 400,
                details: error_messages,
                inner: None,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        application::user::commands::create_user_command::CreateUserCommand,
        infrastructure::rest::request_validation::RequestValidation,
    };

    #[test]
    pub fn error_with_all_fields_invalid() {
        // arrange
        let command = CreateUserCommand {
            first_name: "".to_string(),
            last_name: "".to_string(),
            email: "".to_string(),
            role_id: 0,
            password: "".to_string(),
            is_admin: false,
        };

        // act
        let sut = command.validate().unwrap_err();

        // assert
        assert_eq!(
            sut.message,
            "Error while trying to create a new User".to_string()
        );
        assert_eq!(sut.error_code, 400);
        assert_eq!(
            sut.details,
            vec![
                "First name is empty".to_string(),
                "Last name is empty".to_string(),
                "Email is empty".to_string(),
                "Role ID should be greater than 0".to_string(),
                "First name should have between 3 and 30 characters".to_string(),
                "Last name should have between 3 and 30 characters".to_string(),
                "Empty password".to_string(),
                "Email it's not a valid email address".to_string()
            ]
        );
        assert!(sut.inner.is_none());
    }
}
