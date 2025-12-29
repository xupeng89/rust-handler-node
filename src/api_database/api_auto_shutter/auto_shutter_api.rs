use napi::Result;
use napi_derive::napi;

use crate::error_handle::err_handle::*;
use crate::service_database::database_auto_shutter::service::auto_shutter_service::*;
// use crate::service_database::database_cache::service::auto_shutter_cache_service as cache_service;

// #[napi(namespace = "autoShutter")]
// pub async fn sync_cache_to_local_api() -> Result<()> {
//     let sync_data = cache_service::get_all_model_auto_shutter_entity_cache()
//         .await
//         .map_err(SyncError::CacheReadError)?;

//     if sync_data.is_empty() {
//         eprintln!("⚠️ [同步] 缓存数据库无数据可同步到本地");
//         return Ok(());
//     }

//     // 假设 local_service 接受引用或 clone，保持你原逻辑
//     local_service::read_current_model_auto_shutter_entity(sync_data.clone())
//         .await
//         .map_err(SyncError::LocalWriteError)?;

//     eprintln!("✅ [同步] 成功同步 {} 条数据到本地数据库", sync_data.len());
//     Ok(())
// }

// #[napi(namespace = "autoShutter")]
// pub async fn sync_local_to_cache_api() -> Result<()> {
//     let sync_data = local_service::get_current_all_model_auto_shutter_entity()
//         .await
//         .map_err(SyncError::LocalReadError)?;

//     if sync_data.is_empty() {
//         eprintln!("⚠️ [回写] 本地数据库无数据可回写缓存");
//         return Ok(());
//     }

//     cache_service::read_model_auto_shutter_entity_cache(sync_data.clone())
//         .await
//         .map_err(SyncError::CacheWriteError)?;

//     eprintln!("✅ [回写] 成功回写 {} 条数据到缓存数据库", sync_data.len());
//     Ok(())
// }

// #[napi(namespace = "autoShutter")]
/// 批量插入数据到缓存数据库 (Service: read_model_auto_shutter_entity_cache)
// pub async fn read_auto_shutter_cache_api(data: Vec<FullCacheData>) -> Result<()> {
//     read_model_auto_shutter_entity_cache(data)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(())
// }

#[napi(namespace = "autoShutter")]
/// 更新自动快照 (Service: update_model_auto_shutter_entity_cache)
pub async fn update_auto_shutter_cache_api(data: AutoShutterData, model_id: String) -> Result<i32> {
    let result = update_model_auto_shutter_entity_cache(data, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "autoShutter")]
/// 查询缓存数据库中模型的数据
pub async fn get_all_shutter_cache_api_model_id(model_id: String) -> Result<Vec<FullCacheData>> {
    let result = get_all_model_auto_shutter_entity_cache_model_id(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "autoShutter")]
/// 获取快照列表 (Service: get_all_model_auto_shutter_entity_list_cache)
pub async fn get_auto_shutter_cache_list_api(
    order_flag: String, // "DESC" or "ASC"
    auto_count: u32,
    model_id: String,
) -> Result<Vec<AutoShutterListItem>> {
    // 注意：服务层需要 u64，这里进行转换
    let result =
        get_all_model_auto_shutter_entity_list_cache(order_flag, auto_count as u64, model_id)
            .await
            .map_err(handle_db_err)?;

    Ok(result)
}

#[napi(namespace = "autoShutter")]
/// 获取单个快照详情 (Service: get_model_auto_shutter_entity_by_id_cache)
pub async fn get_auto_shutter_cache_detail_api(id: i32, model_id: String) -> Result<FullCacheData> {
    let result = get_model_auto_shutter_entity_by_id_cache(id, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "autoShutter")]
/// 获取目前快照数量
pub async fn get_all_model_auto_shutter_cache_entity_count(model_id: String) -> Result<u32> {
    let result = get_model_auto_shutter_entity_count(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "autoShutter")]
///插入新的快照记录 (Service: insert_model_auto_shutter_entity_cache)
pub async fn insert_auto_shutter_cache_api(
    data: AutoShutterData,
    model_id: String,
) -> napi::Result<i32> {
    let new_id = read_one_model_auto_shutter_entity_cache(data, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(new_id)
}
