use actix_web::{get, HttpResponse, Result};

use crate::domain::value_objects::hashed_password::HashedPassword;

#[get("validate")]
pub async fn execute() -> Result<HttpResponse> {
    let hashed_password = HashedPassword::new("123123");

    Ok(HttpResponse::Ok().body(hashed_password.unwrap().to_string()))
}
