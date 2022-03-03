use diesel::{EqAll, QueryDsl, RunQueryDsl};
use serde::{Deserialize};
use uuid::Uuid;

use crate::application::clients::queries::get_client::get_client_query_response::GetClientQueryResponse;
use crate::core::base_handler::BaseQueryHandler;
use crate::core::query_error::QueryError;
use crate::domain::clients::client::Client;
use crate::infrastructure::data::db_manager::DbManager;
use crate::schema::clients::dsl::*;

#[derive(Deserialize)]
pub struct GetClientQuery {
    pub id: Uuid,
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

    fn handle(&self, request: &GetClientQuery) -> Result<GetClientQueryResponse, QueryError> {
        let context = &self.conn.read;
        let query = clients.filter(id.eq_all(request.id))
            .first::<Client>(context);

        if query.is_err() {
            return Err(QueryError::Internal);
        }

        let response = GetClientQueryResponse::from(&query.unwrap());

        Ok(response)
    }
}
