use integration_tests::common::ApplicationTestContext;

use crate::integration_tests::application::auth::*;
use crate::integration_tests::application::user::*;

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

    // integration tests, in caso of need to connect to db
    let context = setup().await;
    // let _conn = &context.connection;

    // Auth
    auth_login_query_test::valid_auth_login_query_returns_token().await;
    auth_login_query_test::invalid_credentials_returns_401_and_credentials_were_invalid().await;
    auth_login_query_test::invalid_credentials_returns_401_and_account_not_found_error().await;

    refresh_token_query_test::successful_auth_token_created_based_on_refresh_token().await;
    refresh_token_query_test::empty_refresh_token_throw_error_when_refreshing().await;
    refresh_token_query_test::invalid_refresh_token_throw_error_when_refreshing().await;
    refresh_token_query_test::invalid_sub_prop_from_refresh_token_throw_error_when_refreshing()
        .await;

    // User
    create_user_command_test::successfully_create_new_user().await;
    create_user_command_test::error_while_creating_user_with_existing_email_address().await;
    get_user_query_test::successful_return_an_existing_user().await;

    teardown(context).await;
}
