use chrono::Utc;

use crate::domain::role::role::Role;

#[test]
pub fn create_new_role_model() {
    // arrange
    dotenv::dotenv().ok(); // needed for HashedPassword

    let title = "Admin".to_string();
    let description = Some("Some Description".to_string());

    // act
    let sut = Role::new(title.clone(), description.clone());

    // assert
    assert_eq!(sut.title(), &title);
    assert_eq!(sut.description(), &description);
    assert_eq!(sut.is_active(), &true);
    assert!(sut.updated_at().is_none());
    assert!(sut.created_at().time() < Utc::now().time());
}
