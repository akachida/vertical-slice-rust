use vsa_rust::infrastructure::data::db_manager::DbManager;

pub fn setup() {
    let db_connection = DbManager::new();
}
