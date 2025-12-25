use napi::Result;
use napi_derive::napi;

// 1. 引入错误处理 (假设 SyncError 或类似的枚举已定义)
use crate::error_handle::err_handle::*;
// 2. 引入 Service 层 (包含 ModelDTO, insert_model 等)
use crate::service_database::database_business::service::model_config::model_service as service;
// 3. 引入 DTO 类型用于参数接收
use crate::service_database::database_business::service::model_config::model_service::{
    ModelDTO, ModelUpdateDTO,
};

#[napi(namespace = "modelHandle")]
pub async fn get_model_by_id_api(id: String) -> Result<ModelDTO> {
    let result = service::get_model_by_id(id).await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn has_model_by_id_api(id: String) -> Result<bool> {
    let result = service::has_model_by_id(id).await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn get_all_model_api() -> Result<Vec<ModelDTO>> {
    let result = service::get_all_model().await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn get_first_model_by_update_time_api() -> Result<ModelDTO> {
    let result = service::get_first_model_by_update_time()
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn insert_model_api(model_data: ModelDTO) -> Result<String> {
    // 直接传入 DTO，转换逻辑已在 Service 层的 insert_model 中实现
    let result = service::insert_model(model_data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn update_model_api(model_data: ModelUpdateDTO) -> Result<String> {
    // 直接传入 DTO，Service 层会处理 find_by_id -> into_active_model -> update
    let result = service::update_model(model_data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn delete_model_by_id_api(id: String) -> Result<u32> {
    let result = service::delete_model_by_id(id)
        .await
        .map_err(handle_db_err)?;
    Ok(result as u32)
}

#[napi(namespace = "modelHandle")]
pub async fn select_model_by_no_api(model_no: String) -> Result<String> {
    let result = service::select_model_by_no(model_no)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelHandle")]
pub async fn select_model_by_model_name_api(name: String) -> Result<String> {
    let result = service::select_model_by_name(name)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}
