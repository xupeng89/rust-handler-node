use sea_orm::ConnectionTrait;
use std::time::Duration;
use tokio::sync::OnceCell;
// 使用 sea-orm-migration 提供的 sea_orm，保证类型一致
use sea_orm_migration::MigratorTrait;
use sea_orm_migration::sea_orm as migration_orm;

use crate::service_database::database_cache::migration::Migrator;

// 全局 DB 单例
pub static DB: OnceCell<migration_orm::DatabaseConnection> = OnceCell::const_new();

/// 初始化数据库并执行 migration
pub async fn ensure_cache_db()
-> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    DB.get_or_try_init(|| async {
        let db_url = "sqlite::memory:?cache=shared".to_string();
        let mut opt = migration_orm::ConnectOptions::new(db_url);

        // 2. 核心配置：保持至少一个连接存活
        opt.max_connections(10) // 最大连接数
            .min_connections(1) // 关键：始终保持至少 1 个连接，防止内存被释放
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(None) // 关键：禁止关闭空闲连接
            .max_lifetime(None) // 关键：禁止回收连接（防止重启连接间隙导致数据丢失）
            .sqlx_logging(true); // 调试时开启，可以看到连接池的状态
        // 第一次初始化时跑一次 migration
        let db = migration_orm::Database::connect(opt).await?;

        db.execute_unprepared("PRAGMA journal_mode = WAL;").await?;
        db.execute_unprepared("PRAGMA synchronous = NORMAL;")
            .await?;

        Migrator::up(&db, None).await?;
        eprintln!("✅ [CacheDB] 内存数据库初始化并执行 Migration");

        Ok(db)
    })
    .await
}

/// 获取 DB 连接
pub async fn get_cache_db()
-> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    ensure_cache_db().await
}
