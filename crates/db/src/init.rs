use common::common::DbPool;
use common::config::DatabaseConfig;
use common::error::AppError;
use sea_orm::{ConnectOptions, Database};
use tracing::info;

pub async fn init_db(database_config: &DatabaseConfig) -> Result<DbPool, AppError> {
    let url = format!("sqlite://{}?mode=rwc", database_config.filename);
    let options = ConnectOptions::new(url);
    let connection = Database::connect(options).await?;
    info!("数据库连接初始化成功");
    Ok(connection)
}
