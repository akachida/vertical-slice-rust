pub mod create;
pub mod get;

use actix_web::web;

pub fn user_routes(config: &mut web::ServiceConfig) {
    config
        .service(create::execute)
        .service(get::execute);
}
