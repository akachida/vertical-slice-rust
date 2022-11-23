use actix_web::http::StatusCode;
use backend_api::{
    application::auth::query::{
        auth_login_query::{AuthLoginQuery, AuthLoginQueryHandler},
        refresh_token_query::{RefreshTokenQuery, RefreshTokenQueryHandler},
    },
    domain::{
        user::user::User,
        value_objects::{email::Email, hashed_password::HashedPassword},
    },
    infrastructure::security::jwt::jwt_helper::JwtHelper,
};

pub async fn successful_auth_token_created_based_on_refresh_token() {
    print!("test :: successful_auth_token_created_based_on_refresh_token...");

    // arrange
    let auth_query = AuthLoginQuery {
        username: "admin@master.com".to_string(),
        password: "1aBcD!fg2@".to_string(),
    };
    let auth_query_handler = AuthLoginQueryHandler::new().await.unwrap();
    let auth_query_response = auth_query_handler.handle(&auth_query).await.unwrap();
    let query = RefreshTokenQuery {
        refresh_token: auth_query_response.refresh_token,
    };
    let handler = RefreshTokenQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap();

    // assert
    assert!(!sut.auth_token.is_empty());

    println!("ok");
}

pub async fn empty_refresh_token_throw_error_when_refreshing() {
    print!("test :: empty_refresh_token_throw_error_when_refreshing...");

    // arrange
    let query = RefreshTokenQuery {
        refresh_token: "".to_string(),
    };
    let handler = RefreshTokenQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.error_code, StatusCode::BAD_REQUEST);
    assert_eq!(sut.message, "Refresh token is empty".to_string());
    assert!(sut.details.is_empty());
    assert!(sut.inner.is_none());

    println!("ok");
}

pub async fn invalid_refresh_token_throw_error_when_refreshing() {
    print!("test :: invalid_refresh_token_throw_error_when_refreshing...");

    // arrange
    let query = RefreshTokenQuery {
        refresh_token: "invalid_refresh_token".to_string(),
    };
    let handler = RefreshTokenQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.error_code, StatusCode::BAD_REQUEST);
    assert_eq!(
        sut.message,
        "Refresh token is not valid: Invalid Token".to_string()
    );
    assert!(sut.details.is_empty());
    assert!(sut.inner.is_none());

    println!("ok");
}

pub async fn invalid_sub_prop_from_refresh_token_throw_error_when_refreshing() {
    print!("test :: invalid_sub_prop_from_refresh_token_throw_error_when_refreshing...");

    // arrange
    let unknown_user = User::new(
        "Unknown",
        "User",
        Email::new("unknown@user.com").unwrap(),
        1,
        HashedPassword::new("1ab2c3!2$AB").unwrap(),
        false,
    );
    let refresh_token = JwtHelper::generate_refresh_token(&unknown_user).unwrap();
    let query = RefreshTokenQuery { refresh_token };
    let handler = RefreshTokenQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.error_code, StatusCode::UNAUTHORIZED);
    assert_eq!(sut.message, "Credentials were invalid".to_string());
    assert_eq!(sut.details, vec!["Account not found".to_string()]);
    assert!(sut.inner.is_none());

    println!("ok");
}
