use crate::domain::value_objects::email::{Email, EmailError};

#[test]
fn create_new_email_successfully() {
    // arrange
    let valid_email = "valid@emailaddress.com";

    // act
    let sut = Email::new(valid_email).unwrap();

    // assert
    assert_eq!(sut.to_string(), valid_email);
}

#[test]
fn return_error_when_email_is_invalid() {
    // arrange
    let invalid_email = "not_valid...@1.1.2.3.email_type";

    // act
    let sut = Email::new(invalid_email).unwrap_err();

    // assert
    assert_eq!(sut, EmailError::InvalidEmailFormat);
}

#[test]
fn return_error_when_email_is_empty() {
    // arrange
    let invalid_email = "".to_string();

    // act
    let sut = Email::new(invalid_email.as_str()).unwrap_err();

    // assert
    assert_eq!(sut, EmailError::EmailValueEmpty);
}
