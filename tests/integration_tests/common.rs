use std::env;

use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::fixtures::{roles_fixtures::roles_fixtures, users_fixtures::users_fixtures};

pub struct ApplicationTestContext {
    pub connection: DatabaseConnection,
}

impl ApplicationTestContext {
    pub async fn new() -> Self {
        let db_url = env::var("DATABASE_URL_TEST").expect("env $DATABASE_URL_TEST is not set");
        let db_test_name =
            env::var("TEST_DATABASE_NAME").expect("env $TEST_DATABASE_NAME is not set");

        let connection = sea_orm::Database::connect(format!("{}{}", &db_url, &db_test_name))
            .await
            .unwrap();

        Migrator::down(&connection, None).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        // fixtures
        roles_fixtures(&connection).await;
        users_fixtures(&connection).await;

        Self { connection }
    }

    pub async fn drop(self) {
        Migrator::down(&self.connection, None).await.unwrap();
    }
}
