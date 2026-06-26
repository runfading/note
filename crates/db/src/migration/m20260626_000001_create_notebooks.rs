use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notebooks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notebooks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notebooks::Name).text().not_null())
                    .col(ColumnDef::new(Notebooks::CreatedAt).text().not_null())
                    .col(ColumnDef::new(Notebooks::UpdatedAt).text().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Notebooks {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
