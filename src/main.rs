#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, middleware};

mod application;
mod core;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

use crate::presentation::rest::clients::create_client_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(web::scope("/clients")
                .service(create_client_controller::execute))
        })
        .bind(("localhost", 8080))?
        .run()
        .await
}
