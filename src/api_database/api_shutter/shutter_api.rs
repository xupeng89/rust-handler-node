use napi::Result;
use napi_derive::napi;
// 导入您之前定义的 DTOs 和数据库操作函数
/// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
use crate::error_handle::err_handle::handle_db_err;

use crate::service_database::database_shutter::service::shutter_service::*;
#[napi(namespace = "shutterHandle")]
pub async fn get_all_shutter_entity_list(model_id: String) -> Result<Vec<ShutterListItem>> {
    get_all_model_shutter_entity_list(model_id)
        .await
        .map_err(handle_db_err)
}

/// 创建/更新快照信息 (insertModelShutterEntity)
#[napi(namespace = "shutterHandle")]
pub async fn insert_shutter_entity(data: FullShutterModel, model_id: String) -> Result<i32> {
    insert_model_shutter_entity(data, model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "shutterHandle")]
pub async fn insert_shutter_entity_only(data: FullShutterModel) -> Result<()> {
    insert_model_shutter_entity_only(data)
        .await
        .map_err(handle_db_err)
}

/// 删除快照信息 (deleteModelShutterEntity)
/// 签名: (id: string, modelId: string) => Promise<number> (返回 rows_affected)
#[napi(namespace = "shutterHandle")]
pub async fn delete_shutter_entity(id: String, model_id: String) -> Result<u32> {
    delete_model_shutter_entity(id, model_id)
        .await
        // u64 转换为 u32，假设 rows_affected 不会超过 u32 范围
        .map(|rows| rows as u32)
        .map_err(handle_db_err)
}

#[napi(namespace = "shutterHandle")]
pub async fn get_shutter_entity_by_id(
    id: String,
    model_id: String,
) -> Result<Option<FullShutterModel>> {
    get_model_shutter_entity_by_id(id, model_id)
        .await
        .map_err(handle_db_err)
}

// #[napi(namespace = "shutterHandle")]
// pub async fn get_shutter_entity_by_id_only(id: String) -> Result<Option<FullShutterModel>> {
//     get_model_shutter_entity_by_id_only(id)
//         .await
//         .map_err(handle_db_err)
// }

// 根据 ID 更新部分信息 (updateModelShutterEntityByIndexOnly)
// 签名: (id: string, objects: string, sysvars: string, status: string) => Promise<number> (返回 rows_affected)
#[napi(namespace = "shutterHandle")]
pub async fn update_shutter_entity_by_id_only(
    index_num: i32,
    objects: String,
    sysvars: String,
    status: String,
    model_id: String,
) -> Result<u32> {
    update_model_shutter_entity_by_id_only(index_num, objects, sysvars, status, model_id)
        .await
        .map(|rows| rows as u32)
        .map_err(handle_db_err)
}
