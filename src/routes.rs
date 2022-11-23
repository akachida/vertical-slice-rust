use crate::presentation::rest::{
    auth::{auth_controller, refresh_controller, validate_controller},
    user::create_user_controller,
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

pub fn auth_routes(config: &mut web::ServiceConfig) {
    config
        .service(auth_controller::execute)
        .service(refresh_controller::execute)
        .service(validate_controller::execute);
}

pub fn user_routes(config: &mut web::ServiceConfig) {
    config.service(create_user_controller::execute);
}

//#[post("/private")]
//pub async fn private(user_claims: ReqData<UserClaims>) -> Result<HttpResponse> {
//    dbg!(&user_claims);
//
//    Ok(HttpResponse::Ok().body(serde_json::to_string(&user_claims.into_inner())?))
//}
