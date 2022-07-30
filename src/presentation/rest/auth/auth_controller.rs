use actix_web::{post, web, HttpResponse, Result};
use std::ops::Deref;

use crate::application::auth::query::auth_login_query::{AuthLoginQuery, AuthLoginQueryHandler};

#[post("")]
pub async fn execute(query: web::Json<AuthLoginQuery>) -> Result<HttpResponse> {
    let handler = AuthLoginQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    match handler.unwrap().handle(query.deref()).await {
        Err(error) => error.into_http_response(),
        Ok(result) => Ok(HttpResponse::Ok().body(serde_json::to_string(&result)?)),
    }
}
