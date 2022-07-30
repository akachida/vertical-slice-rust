use crate::{
    infrastructure::security::user_claims::UserClaims,
    presentation::rest::auth::{auth_controller, validate_controller},
};
use actix_web::{
    post,
    web::{self, ReqData},
    HttpResponse, Result,
};

pub fn routes_config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/auth").configure(auth_routes))
        .service(private)
        .route(
            "",
            web::get().to(|| async { HttpResponse::NoContent().await }),
        );
}

pub fn auth_routes(config: &mut web::ServiceConfig) {
    config
        .service(auth_controller::execute)
        .service(validate_controller::execute);
}

#[post("/private")]
pub async fn private(user_claims: ReqData<UserClaims>) -> Result<HttpResponse> {
    dbg!(&user_claims);

    Ok(HttpResponse::Ok().body(serde_json::to_string(&user_claims.into_inner())?))
}
