use uuid::Uuid;
use vsa_rust::application::clients::queries::get_client::get_client_query::{
    GetClientQuery,
    GetClientQueryHandler
};
use vsa_rust::core::base_handler::BaseQueryHandler;

use crate::integration_tests::IntegrationTest;

fn validate_created_client() {
    // arrange
    let uuid_search = Uuid::parse_str("71248280-f6e7-477b-9ef7-8b7b2c4afc2d");

    let query = GetClientQuery {
        id: uuid_search.unwrap(),
    };
    let get_client_query = GetClientQueryHandler::new();

    // act
    let sud = get_client_query.handle(&query)
        .unwrap();

    // assert
    assert_eq!("Client", sud.firstname);
    assert_eq!("Test", sud.lastname);
    assert_eq!("1234567890", sud.document_number);
}

inventory::submit!(IntegrationTest {
    name: "validate_created_client",
    test_fn: validate_created_client
});
