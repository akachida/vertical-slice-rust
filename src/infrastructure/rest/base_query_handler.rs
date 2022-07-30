use async_trait::async_trait;

use super::application_error_response::ApplicationErrorResponse;

#[async_trait]
pub trait BaseQueryHandler<T, R> {
    async fn new() -> Self;
    async fn handle(&self, query: &T) -> Result<R, ApplicationErrorResponse>;
}
