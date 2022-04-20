pub mod domain;

#[derive(Debug)]
pub struct UnitTests {
    pub name: &'static str,
    pub test_fn: fn(),
}

inventory::collect!(UnitTests);
