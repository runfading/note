use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"CREATE VIRTUAL TABLE notes_fts USING fts5(
                title, content,
                content='notes', content_rowid='id',
                tokenize='trigram'
            );"#,
        )
            .await?;

        conn.execute_unprepared(
            r#"CREATE TRIGGER notes_ai AFTER INSERT ON notes BEGIN
                INSERT INTO notes_fts(rowid, title, content)
                VALUES (new.id, new.title, new.content);
            END;"#,
        )
            .await?;

        conn.execute_unprepared(
            r#"CREATE TRIGGER notes_ad AFTER DELETE ON notes BEGIN
                INSERT INTO notes_fts(notes_fts, rowid, title, content)
                VALUES ('delete', old.id, old.title, old.content);
            END;"#,
        )
            .await?;

        conn.execute_unprepared(
            r#"CREATE TRIGGER notes_au AFTER UPDATE ON notes BEGIN
                INSERT INTO notes_fts(notes_fts, rowid, title, content)
                VALUES ('delete', old.id, old.title, old.content);
                INSERT INTO notes_fts(rowid, title, content)
                VALUES (new.id, new.title, new.content);
            END;"#,
        )
            .await?;

        Ok(())
    }
}
