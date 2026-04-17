use crate::api::features::{
    auth::auth_routes,
    user::user_routes,
};
use actix_web::{web, HttpResponse};

pub fn routes_config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/auth").configure(auth_routes))
        .service(web::scope("/user").configure(user_routes))
        .route(
            "",
            web::get().to(|| async { HttpResponse::NoContent().await }),
        );
}
