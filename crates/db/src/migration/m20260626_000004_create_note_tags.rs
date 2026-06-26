use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NoteTags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(NoteTags::NoteId).integer().not_null())
                    .col(ColumnDef::new(NoteTags::TagId).integer().not_null())
                    .col(ColumnDef::new(NoteTags::CreatedAt).text().not_null())
                    .primary_key(
                        Index::create()
                            .name("pk_note_tags")
                            .col(NoteTags::NoteId)
                            .col(NoteTags::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_note_tags_note")
                            .from(NoteTags::Table, NoteTags::NoteId)
                            .to(Notes::Table, Notes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_note_tags_tag")
                            .from(NoteTags::Table, NoteTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum NoteTags {
    Table,
    NoteId,
    TagId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Notes {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
}
