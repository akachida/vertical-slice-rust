use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct ReadAndWriteDatabaseContext {
    pub read: Option<DatabaseConnection>,
    pub write: Option<DatabaseConnection>,
}

#[derive(Debug)]
pub struct ReadDbContext {
    pub conn: Option<DatabaseConnection>,
}

#[derive(Debug)]
pub struct WriteDbContext {
    pub conn: Option<DatabaseConnection>,
}
