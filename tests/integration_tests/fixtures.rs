use diesel::RunQueryDsl;
use diesel::PgConnection;

pub fn clients_fixtures(context: &PgConnection) {
    let query = diesel::sql_query("
        INSERT INTO clients (id, firstname, lastname, document_number)
        VALUES ('71248280-f6e7-477b-9ef7-8b7b2c4afc2d', 'Client', 'Test', '1234567890');");

    query
        .execute(context)
        .expect("[test] Error while running fixtures for clients table");
    }
