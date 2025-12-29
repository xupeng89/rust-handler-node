use sea_orm::entity::*;
// 定义一个 Trait，让不同的 Model 都能返回 ID
pub trait HasId {
    fn get_id(&self) -> i32;
}

pub trait SyncableBinaryEntity: EntityTrait {
    fn col_i() -> String {
        "component_i_id".into()
    }
    fn col_j() -> String {
        "component_j_id".into()
    }
}
// 必须加上 pub
pub trait SyncableBinaryActiveModel: ActiveModelTrait {
    fn set_ids(&mut self, i_id: i32, j_id: i32);
    fn sync_set_from_json(&mut self, json: serde_json::Value) -> Result<(), sea_orm::DbErr>;
}
#[macro_export]
macro_rules! impl_binary_syncable {
    ($model:ty,$active_model:ty, $entity:ty) => {
        impl HasId for $model {
            fn get_id(&self) -> i32 {
                self.id
            } // 假设你的表主键都叫 id
        }
        impl SyncableBinaryEntity for $entity {}

        // 3. 为 ActiveModel 实现设置 ID 的逻辑
        impl $crate::service_database::interface_trait::SyncableBinaryActiveModel
            for $active_model
        {
            fn set_ids(&mut self, i_id: i32, j_id: i32) {
                // 这里的字段名必须与你生成的 ActiveModel 成员名一致
                self.component_i_id = sea_orm::ActiveValue::Set(i_id);
                self.component_j_id = sea_orm::ActiveValue::Set(j_id);
            }
            // 这个方法是公共的
            fn sync_set_from_json(
                &mut self,
                json: serde_json::Value,
            ) -> Result<(), sea_orm::DbErr> {
                let i_id = json["componentIId"].as_i64().unwrap_or(0) as i32;
                let j_id = json["componentJId"].as_i64().unwrap_or(0) as i32;

                self.set_ids(i_id, j_id); // 复用之前定义的 set_ids 方法

                if let Some(v) = json["componentI"].as_str() {
                    self.component_i = sea_orm::ActiveValue::Set(v.to_string());
                }
                if let Some(v) = json["componentJ"].as_str() {
                    self.component_j = sea_orm::ActiveValue::Set(v.to_string());
                }

                Ok(())
            }
        }
    };
}
#[macro_export]
macro_rules! sync_physical_calc_data {
    ($txn:expr, $incoming_ids:expr, $data:expr, $entity:path, $active_model:path, $item:ident, $am:ident, $assign_block:block) => {{
            // A. 删除不在传入列表中的数据
            <$entity as EntityTrait>::delete_many()
                .filter(<$entity as EntityTrait>::Column::Id.is_not_in($incoming_ids.clone()))
                .exec($txn)
                .await?;

            // B. 循环处理每一条数据
            for $item in $data {
                if let Some(id) = $item["id"].as_i64().map(|v| v as i32) {
                    let existing = <$entity as EntityTrait>::find_by_id(id).one($txn).await?;


                    let mut $am = <$active_model as Default>::default();
                    // 统一设置 ID
                    $am.set(<$entity as EntityTrait>::Column::Id, id.into());

                    // 执行外部传入的字段赋值逻辑
                    $assign_block

                    if existing.is_some() {
                        $am.update($txn).await?;
                    } else {
                        $am.insert($txn).await?;
                    }
                }
            }
        }};
}
// 创建一个数据库创建的宏
#[macro_export]
macro_rules! setup_db_instance {
    ($name:ident, $upper_name:ident, $log_prefix:expr, $migrator_path:path) => {
        paste::paste! {
            // 1. 定义该数据库专用的全局单例
            pub static [<$upper_name>]: tokio::sync::OnceCell<sea_orm_migration::sea_orm::DatabaseConnection> = tokio::sync::OnceCell::const_new();

            static [<DB_URL_ $upper_name>]: tokio::sync::OnceCell<String> = tokio::sync::OnceCell::const_new();

            pub async fn [<initialize_ $name _db>](
                file_path: String,
            ) -> Result<&'static sea_orm_migration::sea_orm::DatabaseConnection, sea_orm_migration::sea_orm::DbErr> {
                // 【核心修复】引入 Trait，这样 $migrator_path::up 才能被找到
                use sea_orm_migration::MigratorTrait;
                use sea_orm_migration::sea_orm::ConnectionTrait;

                let path = std::path::Path::new(&file_path);

                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)
                            .map_err(|e| sea_orm_migration::sea_orm::DbErr::Custom(format!("无法创建目录: {}", e)))?;
                    }
                }

                if !path.exists() {
                    std::fs::File::create(path)
                        .map_err(|e| sea_orm_migration::sea_orm::DbErr::Custom(format!("无法创建文件: {}", e)))?;
                }

                let db_url = format!("sqlite://{}?mode=rwc", file_path);
                let _ = [<DB_URL_ $upper_name>].set(db_url);

                [<$upper_name>].get_or_try_init(|| async {
                    let final_url = [<DB_URL_ $upper_name>].get().unwrap();
                    let mut opt = sea_orm_migration::sea_orm::ConnectOptions::new(final_url.to_owned());
                    opt.max_connections(1)
                        .min_connections(1)
                        .connect_timeout(std::time::Duration::from_secs(10))
                        // 设置连接永远不被回收（无限生命周期）
                        .max_lifetime(None)
                        // 设置永远不因空闲而关闭连接
                        .idle_timeout(None)
                        .sqlx_logging(true);

                    let db = sea_orm_migration::sea_orm::Database::connect(opt).await?;

                    // 现在这里不会报错了
                    $migrator_path::up(&db, None).await.map_err(|e| {
                        eprintln!("❌ [{}] Migration 失败: {}", $log_prefix, e);
                        e
                    })?;

                    db.execute_unprepared("PRAGMA journal_mode = WAL;").await?;
                    db.execute_unprepared("PRAGMA synchronous = NORMAL;").await?;

                    eprintln!("✅ [{}] 数据库连接成功", $log_prefix);
                    Ok(db)
                })
                .await
            }

            pub async fn [<get_ $name _db>]() -> Result<&'static sea_orm_migration::sea_orm::DatabaseConnection, sea_orm_migration::sea_orm::DbErr> {
                [<$upper_name>].get().ok_or_else(|| {
                    sea_orm_migration::sea_orm::DbErr::Custom(format!("{} 尚未初始化", $log_prefix))
                })
            }

            pub async fn [<close_ $name _db>]() {
                use sea_orm_migration::sea_orm::ConnectionTrait;
                if let Some(db) = [<$upper_name>].get() {
                    let _ = db.execute_unprepared("PRAGMA wal_checkpoint(TRUNCATE);").await;
                }
                eprintln!("✅ [{}] WAL 已安全落盘", $log_prefix);
            }

            // ==========================================
            // 新增：备份数据库函数
            // ==========================================
            pub async fn [<backup_ $name _db>](target_path: String) -> Result<(), sea_orm_migration::sea_orm::DbErr> {
                use sea_orm_migration::sea_orm::ConnectionTrait;

                // 1. 获取当前连接
                let db = [<get_ $name _db>]().await?;

                // 2. 确保目标目录存在
                let path = std::path::Path::new(&target_path);
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).map_err(|e| sea_orm_migration::sea_orm::DbErr::Custom(format!("无法创建备份目录: {}", e)))?;
                    }
                }

                // 3. 执行 VACUUM INTO
                // 注意：如果目标文件已存在，VACUUM INTO 会报错，所以我们先尝试删除旧备份（可选）
                if path.exists() {
                    let _ = std::fs::remove_file(path);
                }

                let sql = format!("VACUUM INTO '{}'", target_path);
                db.execute_unprepared(&sql).await?;

                eprintln!("✅ [{}] 数据库已备份至: {}", $log_prefix, target_path);
                Ok(())
            }
        }
    };
}
