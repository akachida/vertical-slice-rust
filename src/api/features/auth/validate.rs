use actix_web::{get, HttpResponse, Result};

#[get("validate")]
pub async fn execute() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("OK"))
}
