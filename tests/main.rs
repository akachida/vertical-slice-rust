use integration_tests::common::ApplicationTestContext;

use crate::integration_tests::application::auth::*;

pub mod fixtures;
#[cfg(feature = "integration_tests")]
pub mod integration_tests;

pub async fn setup() -> ApplicationTestContext {
    ApplicationTestContext::new().await
}

async fn teardown(context: ApplicationTestContext) {
    context.drop().await;
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    // integration tests
    let context = setup().await;
    let conn = &context.connection;

    // run each test
    auth_login_query_test::valid_auth_login_query_returns_token(conn).await;
    auth_login_query_test::invalid_credentials_returns_401_unauthorized().await;
    auth_login_query_test::invalid_request_input_dont_pass_validation().await;
    auth_login_query_test::user_not_found_or_invalid_credentials_on_database().await;

    teardown(context).await;
}
