use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20260628_000001_create_note_tags;
mod m20260628_000001_create_notebooks;
mod m20260628_000001_create_notes;
mod m20260628_000001_create_notes_fts;
mod m20260628_000001_create_tags;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260628_000001_create_notes::Migration),
            Box::new(m20260628_000001_create_tags::Migration),
            Box::new(m20260628_000001_create_note_tags::Migration),
            Box::new(m20260628_000001_create_notes_fts::Migration),
            Box::new(m20260628_000001_create_notebooks::Migration),
        ]
    }
}
