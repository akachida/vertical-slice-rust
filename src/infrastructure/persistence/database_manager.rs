use std::env;

use async_trait::async_trait;
use log::warn;

use super::database_context::{ReadAndWriteDatabaseContext, ReadDbContext, WriteDbContext};

#[async_trait]
pub trait DatabaseManager {
    async fn new() -> Self;
    async fn test_db() -> Self;
}

#[async_trait]
impl DatabaseManager for ReadAndWriteDatabaseContext {
    async fn new() -> Self {
        if cfg!(feature = "integration_tests") {
            return Self::test_db().await;
        }

        let read_db_string = env::var("DATABASE_URL_READ").unwrap();
        let write_db_string = env::var("DATABASE_URL_WRITE").unwrap();

        let read_conn = sea_orm::Database::connect(&read_db_string).await;
        let write_conn = sea_orm::Database::connect(&write_db_string).await;

        ReadAndWriteDatabaseContext {
            read: match read_conn {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to read connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
            write: match write_conn {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to write connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }

    async fn test_db() -> Self {
        let url_db = env::var("DATABASE_URL_TEST").unwrap();
        let db_name = env::var("TEST_DATABASE_NAME").unwrap();
        let db_connection_string = format!("{}{}", url_db, db_name);

        let read_conn = sea_orm::Database::connect(&db_connection_string).await;
        let write_conn = sea_orm::Database::connect(&db_connection_string).await;

        ReadAndWriteDatabaseContext {
            read: match read_conn {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to read connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
            write: match write_conn {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to write connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }
}

#[async_trait]
impl DatabaseManager for ReadDbContext {
    async fn new() -> Self {
        if cfg!(feature = "integration_tests") {
            return Self::test_db().await;
        }

        ReadDbContext {
            conn: match sea_orm::Database::connect(&env::var("DATABASE_URL_READ").unwrap()).await {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to read connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }

    async fn test_db() -> Self {
        let url_db = env::var("DATABASE_URL_TEST").unwrap();
        let db_name = env::var("TEST_DATABASE_NAME").unwrap();
        let db_connection_string = format!("{}{}", url_db, db_name);

        ReadDbContext {
            conn: match sea_orm::Database::connect(&db_connection_string).await {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to read connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }
}

#[async_trait]
impl DatabaseManager for WriteDbContext {
    async fn new() -> Self {
        if cfg!(feature = "integration_tests") {
            return Self::test_db().await;
        }

        WriteDbContext {
            conn: match sea_orm::Database::connect(&env::var("DATABASE_URL_WRITE").unwrap()).await {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to write connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }

    async fn test_db() -> Self {
        let url_db = env::var("DATABASE_URL_TEST").unwrap();
        let db_name = env::var("TEST_DATABASE_NAME").unwrap();
        let db_connection_string = format!("{}{}", url_db, db_name);

        WriteDbContext {
            conn: match sea_orm::Database::connect(&db_connection_string).await {
                Err(error) => {
                    warn!(
                        "Error while trying to connect to read connection: {}",
                        error.to_string()
                    );
                    None
                }
                Ok(conn) => Some(conn),
            },
        }
    }
}
