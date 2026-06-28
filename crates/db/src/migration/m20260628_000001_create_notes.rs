use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notes::NotebookId).integer().not_null())
                    .col(ColumnDef::new(Notes::Title).text().not_null())
                    .col(ColumnDef::new(Notes::Content).text().not_null())
                    .col(
                        ColumnDef::new(Notes::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Notes::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_notes_notebook")
                            .from(Notes::Table, Notes::NotebookId)
                            .to(Notebooks::Table, Notebooks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Notes {
    Table,
    Id,
    NotebookId,
    Title,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Notebooks {
    Table,
    Id,
}
