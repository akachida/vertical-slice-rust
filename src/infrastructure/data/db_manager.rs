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

        let write_connection_string = env::var("DATABASE_WRITE_CONNECTION")
            .expect("DATABASE_CONNECTION must be set");

        let read_connection_string = env::var("DATABASE_READ_CONNECTION")
            .expect("DATABASE_CONNECTION must be set");

        DbManager {
            write: PgConnection::establish(&write_connection_string)
                .expect(&format!("Error connecting to {}", write_connection_string)),
            read: PgConnection::establish(&read_connection_string)
                .expect(&format!("Error connecting to {}", read_connection_string)),
        }
    }
}
