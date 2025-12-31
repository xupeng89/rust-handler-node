use sea_orm::ConnectionTrait;
use sea_orm_migration::MigratorTrait;
use sea_orm_migration::sea_orm as migration_orm;
use std::time::Duration;
use tokio::sync::OnceCell;

use crate::service_database::database_cache::migration::Migrator;

// 1. 全局 DB 连接池（业务逻辑使用）
pub static DB: OnceCell<migration_orm::DatabaseConnection> = OnceCell::const_new();
// 2. 关键：守护连接（仅用于防止内存释放）
static DB_GUARD: OnceCell<migration_orm::DatabaseConnection> = OnceCell::const_new();

/// 初始化数据库并执行 migration
pub async fn ensure_cache_db()
-> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    DB.get_or_try_init(|| async {
        // 使用命名的内存数据库 + 共享缓存模式
        // 这允许通过名称访问同一个内存区域
        let db_url = "file:memdb1?mode=memory&cache=shared";

        // --- 步骤 A: 建立守护连接 ---
        // 这个连接没有任何连接池配置，只是为了“占坑”
        DB_GUARD
            .get_or_try_init(|| async { migration_orm::Database::connect(db_url).await })
            .await?;

        // --- 步骤 B: 建立业务连接池 ---
        let mut opt = migration_orm::ConnectOptions::new(db_url);
        opt.max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(10))
            .idle_timeout(None)
            .max_lifetime(None)
            .sqlx_logging(true);

        let db = migration_orm::Database::connect(opt).await?;

        // 初始化配置
        db.execute_unprepared("PRAGMA journal_mode = WAL;").await?;
        db.execute_unprepared("PRAGMA synchronous = NORMAL;")
            .await?;

        // 第一次初始化时跑一次 migration
        Migrator::up(&db, None).await?;
        eprintln!("✅ [CacheDB] 内存数据库已锁定并完成 Migration");

        Ok(db)
    })
    .await
}

pub async fn get_cache_db()
-> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    ensure_cache_db().await
}
