use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::application::clients::queries::get_all_clients::get_all_clients_query_response::GetAllClientsQueryResponse;
use crate::core::base_handler::BaseQueryHandler;
use crate::core::query_error::QueryError;
use crate::domain::clients::client::Client;
use crate::infrastructure::data::db_manager::DbManager;
use crate::schema::clients::dsl::*;

pub struct GetAllClientsQuery {}

pub struct GetAllClientsQueryHandler {
    conn: DbManager,
}

impl BaseQueryHandler<GetAllClientsQuery, Vec<GetAllClientsQueryResponse>>
for GetAllClientsQueryHandler
{
    fn new() -> Self {
        GetAllClientsQueryHandler {
            conn: DbManager::new(),
        }
    }

    fn handle(&self, _: &GetAllClientsQuery)
        -> Result<Vec<GetAllClientsQueryResponse>, QueryError>
    {
        let context = &self.conn.read;
        let result = clients.order(firstname.asc())
            .load::<Client>(context);

        if result.is_err() {
            return Err(QueryError::Internal);
        }

        let response: Vec<GetAllClientsQueryResponse> = result.unwrap()
            .into_iter()
            .map(|x| GetAllClientsQueryResponse::from(x))
            .collect();

        Ok(response)
    }
}
