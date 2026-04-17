pub mod login;
pub mod refresh;
pub mod validate;

use actix_web::web;

pub fn auth_routes(config: &mut web::ServiceConfig) {
    config
        .service(login::execute)
        .service(refresh::execute)
        .service(validate::execute);
}
