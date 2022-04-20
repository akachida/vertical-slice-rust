pub mod common;
pub mod application;
pub mod fixtures;

#[derive(Debug)]
pub struct IntegrationTest {
    pub name: &'static str,
    pub test_fn: fn(),
}

inventory::collect!(IntegrationTest);
