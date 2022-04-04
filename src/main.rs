use vsa_rust::*;
use actix_web::{web, App, HttpServer, middleware};

use crate::presentation::rest::clients::{
    create_client_controller,
    get_client_controller,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(web::scope("/clients")
                .service(create_client_controller::execute)
                .service(get_client_controller::execute))
        })
        .bind(("localhost", 8080))?
        .run()
        .await
}
