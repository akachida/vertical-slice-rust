use vsa_rust::domain::clients::client::Client;

use crate::unit_tests::UnitTests;

fn create_new_client() {
    // arrange & act
    let client = Client::new();

    // assert
    assert_eq!(client.document_number().to_string(), "".to_string());
    assert_eq!(client.firstname().to_string(), "".to_string());
    assert_eq!(client.lastname().to_string(), "".to_string());
}

inventory::submit!(UnitTests {
    name: "create_new_client",
    test_fn: create_new_client
 });

fn set_client_firstname() {
    // arrange
    let mut client = Client::new();

    // act
    let sud = "Ângelo".to_string();
    client.set_firstname(sud.clone());

    // assert
    assert_eq!(sud, client.firstname().to_string());
}

inventory::submit!(UnitTests {
    name: "set_client_firstname",
    test_fn: set_client_firstname
 });

// ... continue the tests over all properties
