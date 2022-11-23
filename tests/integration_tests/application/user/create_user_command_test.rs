use backend_api::application::user::commands::create_user_command::{
    CreateUserCommand, CreateUserCommandHandler,
};

pub async fn successfully_create_new_user() {
    print!("test :: successfully_create_new_user...");

    // arrange
    let command = CreateUserCommand {
        first_name: "New".to_string(),
        last_name: "User".to_string(),
        email: "newuser@domain.com".to_string(),
        role_id: 1,
        password: "a1b2C3D4@!".to_string(),
        is_admin: true,
    };

    let handler = CreateUserCommandHandler::new().await;

    // act
    let sut = handler.unwrap().handle(&command).await;

    // assert
    assert!(sut.is_ok());
    assert!(!sut.unwrap().id.is_nil());

    println!("ok");
}

pub async fn error_while_creating_user_with_existing_email_address() {
    print!("test :: error_while_creating_user_with_existing_email_address...");

    // arrange
    let command = CreateUserCommand {
        first_name: "New".to_string(),
        last_name: "User".to_string(),
        email: "admin@master.com".to_string(),
        role_id: 1,
        password: "a1b2C3D4@!".to_string(),
        is_admin: true,
    };

    let handler = CreateUserCommandHandler::new().await;

    // act
    let sut = handler.unwrap().handle(&command).await.unwrap_err();

    // assert
    assert_eq!(sut.error_code, 400);
    assert_eq!(sut.message, "User already exists".to_string());

    println!("ok");
}
