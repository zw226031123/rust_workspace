use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

/// 数据库连接池类型别名
pub type DbPool = sqlx::Pool<sqlx::Postgres>;

/// 创建数据库连接池
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5) // 最大连接数
        .acquire_timeout(Duration::from_secs(3)) // 连接超时
        .connect(database_url)
        .await
}
