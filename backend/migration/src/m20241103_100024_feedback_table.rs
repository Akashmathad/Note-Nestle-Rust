use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create User table
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create Feedback table
        manager
            .create_table(
                Table::create()
                    .table(Feedback::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Feedback::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Feedback::Title).string().not_null())
                    .col(ColumnDef::new(Feedback::Message).string().not_null())
                    .col(ColumnDef::new(Feedback::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-feedback-user")
                            .from(Feedback::Table, Feedback::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Feedback::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
}

#[derive(DeriveIden)]
enum Feedback {
    Table,
    Id,
    Title,
    Message,
    UserId,
}
