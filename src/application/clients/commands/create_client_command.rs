use diesel::{Insertable, RunQueryDsl};
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

    fn handle(&self, command: &CreateClientCommand) -> bool {
        if command.firstname.is_empty() {
            return false;
        }

        if command.lastname.is_empty() {
            return false;
        }

        if command.document_number.is_empty() {
            return false;
        }

        let context = &self.conn.write;
        let result = diesel::insert_into(clients::table)
            .values(command)
            .execute(context);

        return result.is_ok();
    }
}
