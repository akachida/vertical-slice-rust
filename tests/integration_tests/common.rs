use diesel::RunQueryDsl;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel_migrations::embed_migrations;
use dotenv::dotenv;
use std::env;

use crate::integration_tests::fixtures::clients_fixtures;

pub struct ApplicationTestContext {}

impl ApplicationTestContext {
    pub fn new() -> Self {
        dotenv().ok();

        println!("!! entrou !!");
        let test_connection_string = env::var("DATABASE_TEST_CONNECTION")
            .expect("DATABASE_TEST_CONNECTION must be set");

        println!("!! pegou o var !!");
        let db_connection = PgConnection::establish(&test_connection_string)
            .expect(&format!("[test] Error connecting to {}", test_connection_string));

        // execute migrations
        embed_migrations!("migrations/");
        let _ = embedded_migrations::run(&db_connection);
        println!("!! rodou as migrations !!");

        // start fixtures
        clients_fixtures(&db_connection);

        Self {}
    }
}

impl Drop for ApplicationTestContext {
    fn drop(&mut self) {
        dotenv().ok();
        let db_test_name = "vsa_rust_test";
        let mut test_connection_string = env::var("DATABASE_TEST_CONNECTION")
            .expect("DATABASE_TEST_CONNECTION must be set");
            test_connection_string = test_connection_string.replace(&db_test_name, "");

        let db_connection = PgConnection::establish(&test_connection_string)
            .expect(&format!("[test] Error connecting to {}", &test_connection_string));

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname = '{}'
                AND pid <> pg_backend_pid();",
            &db_test_name
        );

        diesel::sql_query(disconnect_users.as_str())
            .execute(&db_connection)
            .unwrap();

        let query = diesel::sql_query(&format!("DROP DATABASE {};", &db_test_name));

        query
            .execute(&db_connection)
            .expect(&format!("[test] Couldn't drop database {}", &db_test_name));

        let query = diesel::sql_query(&format!("CREATE DATABASE {};", &db_test_name));

        query
            .execute(&db_connection)
            .expect(&format!("[test] Couldn't recreate database {}", &db_test_name));

        println!("!! limpou o db !!");
    }
}
