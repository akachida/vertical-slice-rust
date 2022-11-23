use actix_web::http::StatusCode;
use backend_api::application::auth::query::auth_login_query::{
    AuthLoginQuery, AuthLoginQueryHandler,
};

pub async fn valid_auth_login_query_returns_token() {
    print!("test :: valid_auth_login_query_returns_token...");

    // arrange
    let query = AuthLoginQuery {
        username: "admin@master.com".to_string(),
        password: "1aBcD!fg2@".to_string(),
    };
    let handler = AuthLoginQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap();

    // assert
    assert!(!sut.auth_token.is_empty());
    assert!(!sut.refresh_token.is_empty());

    println!("ok");
}

pub async fn invalid_credentials_returns_401_and_account_not_found_error() {
    print!("test :: invalid_credentials_returns_401_and_account_not_found_error...");

    // arrange
    let query = AuthLoginQuery {
        username: "invalid@email.com".to_string(),
        password: "1aBcD!fg2@".to_string(),
    };
    let handler = AuthLoginQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.message, "Credentials were invalid".to_string());
    assert_eq!(sut.error_code, StatusCode::UNAUTHORIZED);
    assert_eq!(sut.details, vec!["Account not found".to_string()]);
    assert!(sut.inner.is_none());

    println!("ok");
}

pub async fn invalid_credentials_returns_401_and_credentials_were_invalid() {
    print!("test :: invalid_credentials_returns_401_and_credentials_were_invalid...");

    // arrange
    let query = AuthLoginQuery {
        username: "admin@master.com".to_string(),
        password: "1aBcD!fg2@adsasd".to_string(),
    };
    let handler = AuthLoginQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap_err();

    // assert
    assert_eq!(sut.message, "Credentials were invalid".to_string());
    assert_eq!(sut.error_code, StatusCode::UNAUTHORIZED);
    assert_eq!(
        sut.details,
        vec!["Username or password were invalid".to_string()]
    );
    assert!(sut.inner.is_none());

    println!("ok");
}
