use sea_orm_migration::prelude::*;

mod m20260626_000001_create_notebooks;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260626_000001_create_notebooks::Migration)]
    }
}
