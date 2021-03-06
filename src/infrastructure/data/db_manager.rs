use diesel::prelude::*;
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub struct DbManager {
    pub write: PgConnection,
    pub read: PgConnection,
}

impl DbManager {
    pub fn new() -> Self {
        dotenv().ok();

        let write_connection_string: String;
        let read_connection_string: String;

        if cfg!(feature="integration_tests") {
            return DbManager::test_db();
        }

        write_connection_string = env::var("DATABASE_WRITE_CONNECTION")
            .expect("DATABASE_WRITE_CONNECTION must be set");

        read_connection_string = env::var("DATABASE_READ_CONNECTION")
            .expect("DATABASE_READ_CONNECTION must be set");

        DbManager {
            write: PgConnection::establish(&write_connection_string)
                .expect(&format!("Error connecting to {}", write_connection_string)),
            read: PgConnection::establish(&read_connection_string)
                .expect(&format!("Error connecting to {}", read_connection_string)),
        }
    }

    pub fn test_db() -> Self {
        dotenv().ok();

        let test_connection_string = env::var("DATABASE_TEST_CONNECTION")
            .expect("DATABASE_TEST_CONNECTION must be set");

        let write_connection_string = format!("{}", test_connection_string.clone());
        let read_connection_string = format!("{}", test_connection_string.clone());

        DbManager {
            write: PgConnection::establish(&write_connection_string)
                .expect(&format!("Error connecting to {}", write_connection_string)),
            read: PgConnection::establish(&read_connection_string)
                .expect(&format!("Error connecting to {}", read_connection_string)),
        }
    }
}
