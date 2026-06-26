use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

async fn setup_memory_db() -> DatabaseConnection {
    let conn = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
    db::migration::Migrator::up(&conn, None).await.unwrap();
    conn
}

#[tokio::test]
async fn migrator_creates_all_business_tables() {
    let conn = setup_memory_db().await;

    let rows = conn
        .query_all_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT name FROM sqlite_master WHERE type IN ('table','view','trigger') ORDER BY name",
        ))
        .await
        .unwrap();

    let names: Vec<String> = rows
        .into_iter()
        .map(|r| r.try_get_by_index::<String>(0).unwrap())
        .collect();

    let names: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
    assert!(names.contains(&"notebooks"), "missing notebooks table");
    assert!(names.contains(&"tags"), "missing tags table");
    assert!(names.contains(&"notes"), "missing notes table");
    assert!(names.contains(&"note_tags"), "missing note_tags table");
    assert!(names.contains(&"notes_fts"), "missing notes_fts virtual table");
    assert!(names.contains(&"notes_ai"), "missing notes_ai trigger");
    assert!(names.contains(&"notes_ad"), "missing notes_ad trigger");
    assert!(names.contains(&"notes_au"), "missing notes_au trigger");
}

#[tokio::test]
async fn migrator_is_idempotent() {
    let conn = setup_memory_db().await;
    db::migration::Migrator::up(&conn, None).await.unwrap();
}

#[tokio::test]
async fn init_db_creates_file_and_runs_migrations() {
    use common::config::DatabaseConfig;

    let cfg = DatabaseConfig {
        filename: format!("test_init_{}.db", std::process::id()),
    };

    let conn = db::init::init_db(&cfg).await.unwrap();

    let exe = std::env::current_exe().unwrap();
    let file_path = exe.parent().unwrap().join("db").join(&cfg.filename);
    assert!(file_path.exists(), "db file should exist at {:?}", file_path);

    let row = conn
        .query_one_raw(Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS c FROM sqlite_master WHERE type='table' AND name='notes'",
        ))
        .await
        .unwrap()
        .unwrap();
    let count: i64 = row.try_get_by_index(0).unwrap();
    assert_eq!(count, 1, "notes table should exist");

    drop(conn);
    let _ = std::fs::remove_file(&file_path);
    let _ = std::fs::remove_file(file_path.with_extension("db-wal"));
    let _ = std::fs::remove_file(file_path.with_extension("db-shm"));
}
