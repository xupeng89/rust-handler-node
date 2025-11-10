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

/// 获取所有操作记录（JSON 格式）根据model_id
#[napi]
pub async fn list_undo_logs(model_id: String) -> Result<String> {
  list_undo_logs_service(model_id)
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

/// 更新状态（撤销 / 重做）
#[napi]
pub async fn update_undo_status(id: i32, model_id: String, status: i32) -> Result<()> {
  update_status(id, model_id, status)
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  Ok(())
}

/// 删除一条记录
#[napi]
pub async fn delete_undo_logs_by_id(id: i32, model_id: String) -> Result<bool> {
  let delete = delete_undo_logs_service(id, model_id)
    .await
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
  Ok(delete)
}

/// 回滚最近一次的操作记录
/// 重做最近一次的操作记录
