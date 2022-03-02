table! {
    accounts (id) {
        id -> Uuid,
        branch -> Int4,
        number -> Int4,
        suffix -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        client_id -> Uuid,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        document_number -> Varchar,
    }
}

joinable!(accounts -> clients (client_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    clients,
);
