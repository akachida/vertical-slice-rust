use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    user::user::User,
    value_objects::{email::Email, hashed_password::HashedPassword, role::Role},
};

#[test]
pub fn create_new_user_model() {
    // arrange
    dotenv::dotenv().ok(); // needed for HashedPassword

    let first_name = "Ângelo";
    let last_name = "Chida";
    let email = Email::new("valid@email.com").unwrap();
    let role = Role::Admin;
    let password = HashedPassword::new("123123").unwrap();
    let is_admin = true;

    // act
    let sut = User::new(
        first_name,
        last_name,
        email.clone(),
        role,
        password.clone(),
        is_admin,
    );

    // assert
    assert_ne!(sut.id().to_owned(), Uuid::default());
    assert_eq!(sut.first_name().to_owned(), first_name);
    assert_eq!(sut.last_name().to_owned(), last_name);
    assert_eq!(sut.email().to_owned(), email);
    assert_eq!(sut.hashed_password().to_owned(), password);
    assert_eq!(sut.is_admin(), &is_admin);
    assert_eq!(sut.is_active(), &true);
    assert!(sut.updated_at().is_none());
    assert!(sut.created_at().time() < Utc::now().time());
    assert!(sut.last_login_at().is_none());
}
