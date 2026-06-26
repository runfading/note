use sea_orm_migration::prelude::*;

mod m20260626_000001_create_notebooks;
mod m20260626_000002_create_tags;
mod m20260626_000003_create_notes;
mod m20260626_000004_create_note_tags;
mod m20260626_000005_create_notes_fts;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260626_000001_create_notebooks::Migration),
            Box::new(m20260626_000002_create_tags::Migration),
            Box::new(m20260626_000003_create_notes::Migration),
            Box::new(m20260626_000004_create_note_tags::Migration),
            Box::new(m20260626_000005_create_notes_fts::Migration),
        ]
    }
}
