use actix_web::{get, web, HttpResponse, Result};
use uuid::Uuid;

use crate::{
    application::user::queries::get_user_query::{GetUserQuery, GetUserQueryHandler},
    infrastructure::application_error_response::ApplicationErrorResponseTrait,
};

#[get("{id}")]
pub async fn execute(path: web::Path<(Uuid,)>) -> Result<HttpResponse> {
    let handler = GetUserQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let query = GetUserQuery {
        id: path.into_inner().0,
    };
    let response = handler.unwrap().handle(&query).await;

    if response.is_err() {
        return response.unwrap_err().into_http_response();
    }

    Ok(HttpResponse::Ok().body(serde_json::to_string(&response.unwrap())?))
}
