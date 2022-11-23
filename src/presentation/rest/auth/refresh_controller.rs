use actix_web::{post, HttpRequest, HttpResponse, Result};

use crate::{
    application::auth::query::refresh_token_query::{RefreshTokenQuery, RefreshTokenQueryHandler},
    infrastructure::application_error_response::{
        ApplicationErrorResponse, ApplicationErrorResponseTrait,
    },
};

#[post("refresh")]
pub async fn execute(request: HttpRequest) -> Result<HttpResponse> {
    let refresh_cookie = request.cookie("refresh");

    if refresh_cookie.is_none() {
        return ApplicationErrorResponse {
            message: "Refresh token cookie not found".to_string(),
            error_code: 400,
            details: vec![],
            inner: None,
        }
        .into_http_response();
    }

    let handler = RefreshTokenQueryHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let query = RefreshTokenQuery {
        refresh_token: refresh_cookie.unwrap().value().to_string(),
    };

    match handler.unwrap().handle(&query).await {
        Err(error) => error.into_http_response(),
        Ok(result) => Ok(HttpResponse::Ok().body(serde_json::to_string(&result.auth_token)?)),
    }
}
