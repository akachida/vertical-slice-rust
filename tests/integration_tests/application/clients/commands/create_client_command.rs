use vsa_rust::{
    application::clients::commands::create_client_command::{
        CreateClientCommand,
        CreateClientCommandHandler
    },
    core::base_handler::BaseCommandHandler
};

use crate::{integration_tests::IntegrationTest, type_of};

fn create_client_command() {
    // arrange
    let command = CreateClientCommand {
        firstname: "Ângelo".to_string(),
        lastname: "Chida".to_string(),
        document_number: "1234567890".to_string(),
    };

    let handler = CreateClientCommandHandler::new();

    // act
    let sud = handler.handle(&command);

    // assert
    assert!(sud.is_ok());
    assert_eq!("uuid::Uuid", type_of(sud.unwrap()))
}

inventory::submit!(IntegrationTest {
    name: "create_client_command",
    test_fn: create_client_command
});
