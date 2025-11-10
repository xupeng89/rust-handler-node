use crate::database::entity::model_undo_entity_cache::{
  self, ActiveModel, Entity as UndoEntity, Model,
};

use chrono::Utc;
use sea_orm::{entity::*, query::*, ActiveValue::Set, Database, DatabaseConnection, DbErr};

use serde_json;
use tokio::sync::OnceCell;

// ====================
// 全局数据库连接（SQLite 内存模式）
// ====================
static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_db() -> Result<DatabaseConnection, DbErr> {
  DB.get_or_try_init(|| async {
    let db = Database::connect("sqlite::memory:").await?;
    db.execute_unprepared(
      r#"
                CREATE TABLE IF NOT EXISTS model_undo_entity_cache (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    model_id TEXT NOT NULL,
                    table_name TEXT NOT NULL,
                    op_type TEXT NOT NULL,
                    old_data TEXT,
                    new_data TEXT,
                    status INTEGER DEFAULT 0,
                    operator_at TEXT DEFAULT CURRENT_TIMESTAMP
                );
            "#,
    )
    .await?;
    Ok::<_, DbErr>(db)
  })
  .await
  .cloned()
}

// ====================
// 服务函数实现
// ====================

pub async fn add_undo_log_service(
  op_type: String,
  table_name: String,
  old_data: String,
  new_data: String,
  model_id: String,
) -> Result<(), DbErr> {
  let db = get_db().await?;

  let new_item = model_undo_entity_cache::ActiveModel {
    model_id: Set(model_id),
    table_name: Set(table_name),
    op_type: Set(op_type),
    old_data: Set(old_data),
    new_data: Set(new_data),
    status: Set(0),
    operator_at: Set(Utc::now().naive_utc()),
    ..Default::default()
  };

  new_item.insert(&db).await?;
  Ok(())
}

pub async fn list_undo_logs_service() -> Result<String, DbErr> {
  let db = get_db().await?;
  let logs: Vec<Model> = UndoEntity::find().all(&db).await?;
  Ok(serde_json::to_string(&logs).unwrap())
}

pub async fn update_status_service(id: i32, status: i32) -> Result<(), DbErr> {
  let db = get_db().await?;

  if let Some(record) = UndoEntity::find_by_id(id).one(&db).await? {
    let mut active: ActiveModel = record.into();
    active.status = Set(status);
    active.update(&db).await?;
  }

  Ok(())
}
