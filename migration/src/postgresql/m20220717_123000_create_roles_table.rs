use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220717_123000_create_roles_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .small_integer()
                            .unique_key()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Role::Title).string_len(20).not_null())
                    .col(ColumnDef::new(Role::Description).string_len(128))
                    .col(ColumnDef::new(Role::IsActive).boolean().default(true))
                    .col(ColumnDef::new(Role::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(Role::CreatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
    Title,
    Description,
    IsActive,
    UpdatedAt,
    CreatedAt,
}
