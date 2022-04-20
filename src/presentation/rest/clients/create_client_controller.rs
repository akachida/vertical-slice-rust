use std::ops::Deref;
use actix_web::{Result, HttpResponse, post, web};
use crate::application::clients::commands::create_client_command::{
    CreateClientCommand,
    CreateClientCommandHandler
};
use crate::core::base_handler::BaseCommandHandler;

#[post("/create")]
pub async fn execute(command: web::Json<CreateClientCommand>)
    -> Result<HttpResponse>
{
    let handler = CreateClientCommandHandler::new();
    let response = handler.handle(command.deref());

    if response.is_ok() {
        return Ok(HttpResponse::Created().body("201 Created"));
    }

    Ok(HttpResponse::BadRequest().body("401 Bad Request"))
}
