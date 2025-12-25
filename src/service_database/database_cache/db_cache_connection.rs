use std::time::Duration;
use tokio::sync::OnceCell;

// 使用 sea-orm-migration 提供的 sea_orm，保证类型一致
use sea_orm_migration::sea_orm as migration_orm;
use sea_orm_migration::MigratorTrait;

use crate::service_database::database_cache::migration::Migrator;

// 全局 DB 单例
pub static DB: OnceCell<migration_orm::DatabaseConnection> = OnceCell::const_new();

/// 初始化数据库并执行 migration
pub async fn ensure_cache_db(
) -> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    DB.get_or_try_init(|| async {
        // SQLite 数据库 URL
        // let db_url = "sqlite:app.db?mode=rwc".to_string();
        let db_url = "sqlite::memory:?cache=shared".to_string();
        // 创建连接参数
        let mut opt = migration_orm::ConnectOptions::new(db_url);
        opt.max_connections(1)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(5))
            .sqlx_logging(true);

        // 创建连接（注意使用 migration_orm::Database）
        let db = migration_orm::Database::connect(opt).await?;

        // 运行 migration（最重要：必须传 2 个参数）
        Migrator::up(&db, None).await?;
        eprintln!("✅ [CacheDB] 数据库连接成功");
        Ok::<migration_orm::DatabaseConnection, migration_orm::DbErr>(db)
    })
    .await
}

/// 获取 DB 连接
pub async fn get_cache_db(
) -> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    ensure_cache_db().await
}
