use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create Subject table
        manager
            .create_table(
                Table::create()
                    .table(Subject::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subject::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Subject::Name).string().not_null())
                    .col(ColumnDef::new(Subject::Branch).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Create Unit table
        manager
            .create_table(
                Table::create()
                    .table(Unit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Unit::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(Unit::Name).string().not_null())
                    .col(ColumnDef::new(Unit::SubjectId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-unit-subject")
                            .from(Unit::Table, Unit::SubjectId)
                            .to(Subject::Table, Subject::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create File table
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()")),
                    )
                    .col(ColumnDef::new(File::Name).string().not_null())
                    .col(ColumnDef::new(File::Owner).string().not_null())
                    .col(ColumnDef::new(File::Url).string())
                    .col(ColumnDef::new(File::UnitId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-file-unit")
                            .from(File::Table, File::UnitId)
                            .to(Unit::Table, Unit::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop File table
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        // Drop Unit table
        manager
            .drop_table(Table::drop().table(Unit::Table).to_owned())
            .await?;

        // Drop Subject table
        manager
            .drop_table(Table::drop().table(Subject::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Subject {
    Table,
    Id,
    Name,
    Branch,
}

#[derive(DeriveIden)]
enum Unit {
    Table,
    Id,
    Name,
    SubjectId,
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    Name,
    Owner,
    Url,
    UnitId,
}
