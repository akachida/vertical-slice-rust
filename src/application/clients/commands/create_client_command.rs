use diesel::{Insertable, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use crate::core::base_handler::BaseCommandHandler;
use crate::core::query_error::QueryError;
use crate::domain::clients::client::Client;
use crate::infrastructure::data::db_manager::DbManager;
use crate::schema::clients;

#[derive(Deserialize, Insertable)]
#[table_name="clients"]
pub struct CreateClientCommand {
    pub firstname: String,
    pub lastname: String,
    pub document_number: String,
}

pub struct CreateClientCommandHandler {
    conn: DbManager,
}

impl BaseCommandHandler<CreateClientCommand, Uuid> for CreateClientCommandHandler {
    fn new() -> Self {
        CreateClientCommandHandler {
            conn: DbManager::new(),
        }
    }

    fn handle(&self, command: &CreateClientCommand) -> Result<Uuid, QueryError> {
        if command.firstname.is_empty() {
            return Err(QueryError::InvalidParameter);
        }

        if command.lastname.is_empty() {
            return Err(QueryError::InvalidParameter);
        }

        if command.document_number.is_empty() {
            return Err(QueryError::InvalidParameter);
        }

        let context = &self.conn.write;
        let result = diesel::insert_into(clients::table)
            .values(command)
            .get_result::<Client>(context);

        if result.is_err() {
            return Err(QueryError::Internal);
        }

        let new_client: Client = result.unwrap();

        Ok(*new_client.id())
    }
}
