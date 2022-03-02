use crate::core::query_error::QueryError;

pub trait BaseCommandHandler<T> {
    fn new() -> Self;
    fn handle(&self, command: &T) -> bool;
}

pub trait BaseQueryHandler<T, R> {
    fn new() -> Self;
    fn handle(&self, request: &T) -> Result<R, QueryError>;
}
