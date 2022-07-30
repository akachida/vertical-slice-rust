use sea_orm_migration::prelude::*;

use super::m20220717_123000_create_roles_table::Role;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220717_123700_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().unique_key().primary_key())
                    .col(ColumnDef::new(User::FirstName).string_len(25).not_null())
                    .col(ColumnDef::new(User::LastName).string_len(25).not_null())
                    .col(
                        ColumnDef::new(User::Email)
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::RoleId).small_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-role-to-role_id")
                            .from(User::Table, User::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(
                        ColumnDef::new(User::HashedPassword)
                            .not_null()
                            .string_len(256),
                    )
                    .col(ColumnDef::new(User::IsActive).boolean().default(true))
                    .col(ColumnDef::new(User::IsAdmin).boolean().default(false))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::LastLoginAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Email,
    RoleId,
    HashedPassword,
    IsActive,
    IsAdmin,
    UpdatedAt,
    CreatedAt,
    LastLoginAt,
}
