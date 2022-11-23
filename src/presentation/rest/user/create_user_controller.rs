use actix_web::{post, web, HttpResponse, Result};

use crate::{
    application::user::commands::create_user_command::{
        CreateUserCommand, CreateUserCommandHandler,
    },
    infrastructure::application_error_response::ApplicationErrorResponseTrait,
};

#[post("")]
pub async fn execute(command: web::Json<CreateUserCommand>) -> Result<HttpResponse> {
    let handler = CreateUserCommandHandler::new().await;

    if handler.is_err() {
        return handler.unwrap_err().into_http_response();
    }

    let handle = handler.unwrap().handle(&command).await;

    if handle.is_err() {
        return handle.unwrap_err().into_http_response();
    }

    let response = handle.unwrap();
    // todo: create a URL location for resource created
    // let url_for = HttpRequest::url_for();

    Ok(HttpResponse::Created()
        .append_header(("Location", format!("/api/user/{}", response.id.to_owned())))
        .body(response.id.to_string()))
}
