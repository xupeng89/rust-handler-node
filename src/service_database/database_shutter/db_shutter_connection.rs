use std::fs::{self, File};
use std::path::Path;
use std::time::Duration;
use tokio::sync::OnceCell;

use sea_orm_migration::sea_orm as migration_orm;
use sea_orm_migration::MigratorTrait;

// 确保引入了正确的 Migrator
use crate::service_database::database_shutter::migration::Migrator;
use migration_orm::ConnectionTrait;

// 全局 DB 连接单例
pub static DB: OnceCell<migration_orm::DatabaseConnection> = OnceCell::const_new();
static DB_URL: OnceCell<String> = OnceCell::const_new();

/// **公共初始化入口**
pub async fn initialize_shutter_db(
    file_path: String, // 传入文件路径，例如: "/app/data/shutter.db"
) -> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    // ---------------------------------------------------------
    // 1. 增强的文件系统处理 (确保目录和文件存在)
    // ---------------------------------------------------------
    let path = Path::new(&file_path);

    // 自动创建父目录
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            eprintln!("📂 [ShutterDB] 父目录不存在，正在创建: {:?}", parent);
            fs::create_dir_all(parent)
                .map_err(|e| migration_orm::DbErr::Custom(format!("无法创建数据库目录: {}", e)))?;
        }
    }

    // 如果文件不存在，手动创建一个空文件 (虽然 sqlite mode=rwc 会做，但显式创建更稳健)
    if !path.exists() {
        eprintln!("🆕 [ShutterDB] 数据库文件不存在，创建新文件: {:?}", path);
        File::create(path)
            .map_err(|e| migration_orm::DbErr::Custom(format!("无法创建数据库文件: {}", e)))?;
    }

    // 构造 SQLite 连接字符串
    // 注意: 使用 protocol 格式，确保路径正确转义
    // mode=rwc: 读写创建
    let db_url = format!("sqlite://{}?mode=rwc", file_path);
    DB_URL.set(db_url.clone()).ok();

    DB.get_or_try_init(|| async {
        let final_db_url = DB_URL.get().unwrap().as_str();

        let mut opt = migration_orm::ConnectOptions::new(final_db_url.to_owned());
        opt.max_connections(16)
            .min_connections(4)
            .connect_timeout(Duration::from_secs(10))
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .sqlx_logging(true); // 开发调试时建议开启

        // 2. 创建连接
        let db = migration_orm::Database::connect(opt).await?;

        // 3. 运行 Migration (核心步骤)
        Migrator::up(&db, None).await.map_err(|e| {
            eprintln!("❌ [ShutterDB] Migration 失败: {}", e);
            e
        })?;

        // 4. 设置 WAL 模式
        db.execute_unprepared("PRAGMA journal_mode = WAL;").await?;
        db.execute_unprepared("PRAGMA synchronous = NORMAL;")
            .await?;
        eprintln!("✅ [ShutterDB] 数据库连接成功");
        Ok::<migration_orm::DatabaseConnection, migration_orm::DbErr>(db)
    })
    .await
}

// get_shutter_db 保持不变...
pub async fn get_shutter_db(
) -> Result<&'static migration_orm::DatabaseConnection, migration_orm::DbErr> {
    match DB.get() {
        Some(db_conn) => Ok(db_conn),
        None => Err(migration_orm::DbErr::Custom(
            "ShutterDB Database not initialized.".to_string(),
        )),
    }
}

pub async fn close_shutter_db() {
    // 2. 仅 SQLite 执行 WAL 清理操作
    if let Some(db) = DB.get() {
        // WAL 强制落盘
        if let Err(e) = db
            .execute_unprepared("PRAGMA wal_checkpoint(TRUNCATE);")
            .await
        {
            eprintln!("⚠️ [ShutterDB] WAL checkpoint 失败: {}", e);
        }

        // 可选：降低同步级别，进入只读 / 退出态
        let _ = db.execute_unprepared("PRAGMA synchronous = FULL;").await;
    }

    eprintln!("✅ [ShutterDB] WAL 已安全落盘");
}
