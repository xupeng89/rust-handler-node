#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
mod database;
use database::undo_redo_service::*;

/// 插入一条操作记录
#[napi]
pub async fn add_undo_log(
  op_type: String,
  table_name: String,
  old_data: String,
  new_data: String,
  model_id: String,
) -> Result<()> {
  add_undo_log_service(op_type, table_name, old_data, new_data, model_id)
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  Ok(())
}

/// 获取所有操作记录（JSON 格式）
#[napi]
pub async fn list_undo_logs() -> Result<String> {
  list_undo_logs_service()
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

/// 更新状态（撤销 / 重做）
#[napi]
pub async fn update_undo_status(id: i32, status: i32) -> Result<()> {
  update_status_service(id, status)
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  Ok(())
}
