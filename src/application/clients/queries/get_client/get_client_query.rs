use serde::{Deserialize};

use crate::application::clients::queries::get_client::get_client_query_response::GetClientQueryResponse;
use crate::core::base_handler::BaseQueryHandler;
use crate::core::query_error::QueryError;
use crate::infrastructure::data::db_manager::DbManager;

#[derive(Deserialize)]
pub struct GetClientQuery {
    pub id: String,
}

pub struct GetClientQueryHandler {
    conn: DbManager,
}

impl BaseQueryHandler<GetClientQuery, GetClientQueryResponse>
    for GetClientQueryHandler
{
    fn new() -> Self {
        GetClientQueryHandler {
            conn: DbManager::new(),
        }
    }

    fn handle(&self, _: &GetClientQuery) -> Result<GetClientQueryResponse, QueryError> {
        todo!()
    }
}
