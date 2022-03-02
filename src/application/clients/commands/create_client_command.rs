use diesel::{Insertable};
use serde::{Deserialize};
use crate::core::base_handler::BaseCommandHandler;
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

impl BaseCommandHandler<CreateClientCommand> for CreateClientCommandHandler {
    fn new() -> Self {
        CreateClientCommandHandler {
            conn: DbManager::new(),
        }
    }

    fn handle(&self, _: &CreateClientCommand) -> bool {
        true
    }
}
