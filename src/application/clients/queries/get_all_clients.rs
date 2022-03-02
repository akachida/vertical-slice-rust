use crate::core::base_handler::BaseQueryHandler;
use crate::core::query_error::QueryError;
use crate::infrastructure::data::db_manager::DbManager;

pub struct GetAllClientsQuery {}

pub struct GetAllClientsQueryResponse {}

pub struct GetAllClientsQueryHandler {
    conn: DbManager,
}

impl BaseQueryHandler<GetAllClientsQuery, GetAllClientsQueryResponse>
for GetAllClientsQueryHandler
{
    fn new() -> Self {
        GetAllClientsQueryHandler {
            conn: DbManager::new(),
        }
    }

    fn handle(&self, _: &GetAllClientsQuery) -> Result<GetAllClientsQueryResponse, QueryError> {
        todo!()
    }
}
