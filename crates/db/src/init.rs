use common::common::DbPool;
use common::config::DatabaseConfig;
use common::error::AppError;
use sea_orm::{ConnectOptions, ConnectionTrait, Database};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use crate::migration::Migrator;
use crate::path;

pub async fn init_db(cfg: &DatabaseConfig) -> Result<DbPool, AppError> {
    let db_path = path::resolve_db_path(&cfg.filename)
        .map_err(|e| AppError::NotFound(format!("resolve db path: {e}")))?;
    let url = format!("sqlite://{}?mode=rwc", db_path.display());
    info!("数据库文件路径: {}", db_path.display());

    let mut options = ConnectOptions::new(url);
    options.sqlx_logging(true);
    let conn = Database::connect(options).await?;

    conn.execute_unprepared("PRAGMA journal_mode=WAL").await?;
    conn.execute_unprepared("PRAGMA foreign_keys=ON").await?;
    conn.execute_unprepared("PRAGMA synchronous=NORMAL").await?;

    Migrator::up(&conn, None).await?;
    info!("数据库初始化成功");
    Ok(conn)
}
