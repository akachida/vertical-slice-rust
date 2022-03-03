use actix_web::{Result, HttpResponse, get, web};
use uuid::Uuid;

use crate::application::clients::queries::get_client::get_client_query::{GetClientQuery, GetClientQueryHandler};
use crate::core::base_handler::BaseQueryHandler;
use crate::core::query_error::QueryError;

#[get("/{uuid}")]
async fn execute(uuid: web::Path<String>) -> Result<HttpResponse> {
    let mut request = GetClientQuery { id: Uuid::nil() };
    let uuid = Uuid::parse_str(uuid.to_string().as_mut_str());

    match uuid {
        Ok(id) => { request.id = id },
        Err(e) => return Ok(HttpResponse::BadRequest().body(e.to_string()))
    }

    let handler = GetClientQueryHandler::new();
    let response = handler.handle(&request);

    match response {
        Ok(data) =>
            Ok(HttpResponse::Ok().body(serde_json::to_string(&data).unwrap().to_string())),
        Err(e) => match e {
            QueryError::Internal => Ok(HttpResponse::InternalServerError().body(e.to_string())),
            QueryError::NotFound => Ok(HttpResponse::NotFound().body(e.to_string()))
        }
    }
}
