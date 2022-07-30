pub use sea_orm_migration::prelude::*;

mod postgresql;

use postgresql::m20220717_123000_create_roles_table;
use postgresql::m20220717_123700_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220717_123000_create_roles_table::Migration),
            Box::new(m20220717_123700_create_users_table::Migration),
        ]
    }
}
