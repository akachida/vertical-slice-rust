#[macro_use] extern crate diesel_migrations;

#[cfg(feature = "integration_tests")]
pub mod integration_tests;
pub mod unit_tests;

use integration_tests::{common::ApplicationTestContext, IntegrationTest};
use std::any::type_name;
use unit_tests::UnitTests;

fn setup() -> ApplicationTestContext {
    ApplicationTestContext::new()
}

fn teardown(context: &ApplicationTestContext) {
    drop(context);
}

fn main() {
    // integration tests
    let context = setup();

    for t in inventory::iter::<IntegrationTest> {
        println!("Running integration test: {}", t.name);
        (t.test_fn)()
    }

    teardown(&context);

    // unit tests
    for t in inventory::iter::<UnitTests> {
        println!("Running unit test: {}", t.name);
        (t.test_fn)()
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
