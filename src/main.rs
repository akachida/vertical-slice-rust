#[macro_use]
extern crate log;
extern crate chrono;
extern crate dotenv;

use std::env;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use backend_api::*;
use migration::{Migrator, MigratorTrait};
use routes::routes_config;

use crate::infrastructure::middleware::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    env::var("AUTH_SECRET").expect("env $AUTH_SECRET is not set");
    env::var("REFRESH_TOKEN_SECRET").expect("env $REFRESH_TOKEN_SECRET is not set");
    env::var("DATABASE_URL_READ").expect("env $DATABASE_URL_READ is not set");
    env::var("DATABASE_URL_TEST").expect("env $DATABASE_URL_TEST is not set");
    env::var("TEST_DATABASE_NAME").expect("env $TEST_DATABASE_NAME is not set");
    env::var("PASSWORD_SECRET").expect("env $TEST_DATABASE_NAME is not set");

    let db_url = env::var("DATABASE_URL_WRITE").expect("env $DATABASE_URL_WRITE is not set");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    info!("Starting up server...");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().add(("Content-type", "application/json")))
            .wrap(middleware::DefaultHeaders::new().add(("Content-Security-Policy", "default-src 'self'")))
            .wrap(middleware::DefaultHeaders::new().add(("X-Content-Type-Options", "nosniff")))
            .wrap(middleware::DefaultHeaders::new().add(("X-Xss-Protection", "1; mode=block")))
            .wrap(middleware::DefaultHeaders::new().add(("X-Permitted-Cross-Domain-Policies", "none")))
            .wrap(authentication_middleware::Authentication)
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(routes_config))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::NoContent().await }),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
