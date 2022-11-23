use std::str::FromStr;

use backend_api::application::user::queries::get_user_query::{GetUserQuery, GetUserQueryHandler};
use uuid::Uuid;

pub async fn successful_return_an_existing_user() {
    print!("test :: successful_return_an_existing_user...");

    // arrange
    let query = GetUserQuery {
        id: Uuid::from_str("425123f7-49c6-44f4-b39e-31477092a8c3").unwrap(),
    };
    let handler = GetUserQueryHandler::new().await.unwrap();

    // act
    let sut = handler.handle(&query).await.unwrap();

    // assert
    assert_eq!(sut.id, query.id);
    assert_eq!(sut.first_name, "Admin".to_string());
    assert_eq!(sut.last_name, "Master".to_string());

    println!("ok");
}
