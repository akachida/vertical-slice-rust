use actix_web::http::StatusCode;
use backend_api::{
    application::auth::query::auth_login_query::{AuthLoginQuery, AuthLoginQueryHandler},
    domain::user::user::UserTrait,
    infrastructure::security::jwt::jwt_helper::JwtHelper,
};
use entity::user;
use entity::user::Entity as User;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn valid_auth_login_query_returns_token(context: &DatabaseConnection) {
    print!("test :: valid_auth_login_query_returns_token...");
    // arrange
    let query = AuthLoginQuery {
        username: "admin@master.com".to_string(),
        password: "123123".to_string(),
    };
    let handler = AuthLoginQueryHandler::new().await.unwrap();
    let user_find = User::find()
        .filter(user::Column::Email.contains(&query.username))
        .one(context)
        .await
        .unwrap()
        .unwrap();
    let expected_token = JwtHelper::generate(&user_find.into_domain()).unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap();

    // assert
    assert!(!sut.token.is_empty());
    assert_eq!(expected_token, sut.token);

    println!("ok");
}

pub async fn invalid_credentials_returns_401_unauthorized() {
    print!("test :: invalid_credentials_returns_401_unauthorized...");
    // arrange
    let query = AuthLoginQuery {
        username: "invalid@email.com".to_string(),
        password: "123123".to_string(),
    };
    let handler = AuthLoginQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.message, "Credentials were invalid".to_string());
    assert_eq!(sut.error_code, StatusCode::UNAUTHORIZED);
    assert_eq!(
        sut.details,
        vec![
            "Username or password were invalid".to_string(),
            "Account not found".to_string(),
        ]
    );
    assert!(sut.inner.is_none());

    println!("ok");
}