use crate::api_cache::err_handle::handle_db_err;
use crate::database_cache::service::undo_redo_service::*;
use napi::*;
use napi_derive::napi;

// 插入一条操作记录
#[napi(namespace = "undoRedoHandle")]
/// 增加一条记录
pub async fn add_undo_log(
    op_type: String,
    table_name: String,
    old_data: String,
    new_data: String,
    model_id: String,
) -> Result<i32> {
    let id = add_undo_log_service(op_type, table_name, old_data, new_data, model_id)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    Ok(id)
}

// 获取所有操作记录（JSON 格式）根据model_id
#[napi(namespace = "undoRedoHandle")]
/// 根据modelId获取撤回操作的所有信息
pub async fn list_undo_logs(model_id: String) -> Result<Vec<UndoRedoData>> {
    let result = list_undo_logs_service(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 更新状态（撤销 / 重做）
#[napi(namespace = "undoRedoHandle")]
/// 更新 状态，用于处理下一步操作
pub async fn update_undo_status(id: i32, model_id: String, status: i32) -> Result<()> {
    update_status_service(id, model_id, status)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

// 删除一条记录
#[napi(namespace = "undoRedoHandle")]
/// 清除已经使用过的信息，防止来回操作
pub async fn delete_undo_logs_by_id(id: i32, model_id: String) -> Result<bool> {
    let delete = delete_undo_log_service(id, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(delete)
}

// 删除所有记录
#[napi(namespace = "undoRedoHandle")]
/// 清除所有回撤记录
pub async fn delete_undo_logs_by_model_id(model_id: String) -> Result<bool> {
    let delete = delete_undo_log_service_by_model_id(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(delete)
}

// 撤销 -- 最近一次的操作记录
#[napi(namespace = "undoRedoHandle")]
pub async fn get_the_last_undo_model(model_id: String) -> Result<UndoRedoData> {
    let error_model_id = model_id.clone();
    let result = get_latest_pending_log(model_id)
        .await
        .map_err(handle_db_err)?;
    match result {
        Some(data) => Ok(data),
        None => Err(Error::new(
            Status::GenericFailure,
            format!("未找到 model_id: {} 对应的待处理日志", error_model_id),
        )),
    }
}
// 重做 -- 最近一次的操作记录
#[napi(namespace = "undoRedoHandle")]
// 获取最后一条重做信息
pub async fn get_the_last_redo_model(model_id: String) -> Result<UndoRedoData> {
    let result = get_latest_redo_log(model_id.clone())
        .await
        .map_err(handle_db_err)?;
    // 显式处理 None：如果查询结果为空，抛出一个自定义错误
    match result {
        Some(data) => Ok(data),
        None => Err(Error::new(
            Status::GenericFailure,
            format!("未找到 model_id: {} 对应的待处理日志", model_id),
        )),
    }
}

#[napi(namespace = "undoRedoHandle")]
/// 更新一条信息状态
pub async fn update_undo_new_data(id: i32, model_id: String, new_data: String) -> Result<()> {
    update_new_data_service(id, model_id, new_data)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;
    Ok(())
}

#[napi(namespace = "undoRedoHandle")]
pub async fn test() -> Result<()> {
    bench_insert()
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}
