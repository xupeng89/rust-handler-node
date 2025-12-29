// use napi::Result;
// use napi_derive::napi;

// // 引入服务层模块和 DTO/类型
// use crate::service_database::database_cache::service::auto_shutter_cache_service as service;
// use service::{AutoShutterData, AutoShutterListItem, FullCacheData};

// /// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
// use crate::error_handle::err_handle::handle_db_err;

// #[napi(namespace = "autoShutterCache")]
// /// 批量插入数据到缓存数据库 (Service: read_model_auto_shutter_entity_cache)
// pub async fn read_auto_shutter_cache_api(data: Vec<FullCacheData>) -> Result<()> {
//     service::read_model_auto_shutter_entity_cache(data)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(())
// }

// #[napi(namespace = "autoShutterCache")]
// /// 更新自动快照 (Service: update_model_auto_shutter_entity_cache)
// pub async fn update_auto_shutter_cache_api(data: AutoShutterData, model_id: String) -> Result<i32> {
//     let result = service::update_model_auto_shutter_entity_cache(data, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutterCache")]
// /// 查询缓存数据库中模型的数据
// pub async fn get_all_shutter_cache_api_model_id(model_id: String) -> Result<Vec<FullCacheData>> {
//     let result = service::get_all_model_auto_shutter_entity_cache_model_id(model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutterCache")]
// /// 获取快照列表 (Service: get_all_model_auto_shutter_entity_list_cache)
// pub async fn get_auto_shutter_cache_list_api(
//     order_flag: String, // "DESC" or "ASC"
//     auto_count: u32,
//     model_id: String,
// ) -> Result<Vec<AutoShutterListItem>> {
//     // 注意：服务层需要 u64，这里进行转换
//     let result = service::get_all_model_auto_shutter_entity_list_cache(
//         order_flag,
//         auto_count as u64,
//         model_id,
//     )
//     .await
//     .map_err(handle_db_err)?;

//     Ok(result)
// }

// #[napi(namespace = "autoShutterCache")]
// /// 获取单个快照详情 (Service: get_model_auto_shutter_entity_by_id_cache)
// pub async fn get_auto_shutter_cache_detail_api(id: i32, model_id: String) -> Result<FullCacheData> {
//     let result = service::get_model_auto_shutter_entity_by_id_cache(id, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutterCache")]
// /// 获取目前快照数量
// pub async fn get_all_model_auto_shutter_cache_entity_count(model_id: String) -> Result<u32> {
//     let result = service::get_model_auto_shutter_entity_count(model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutterCache")]
// ///插入新的快照记录 (Service: insert_model_auto_shutter_entity_cache)
// pub async fn insert_auto_shutter_cache_api(
//     data: AutoShutterData,
//     model_id: String,
// ) -> napi::Result<i32> {
//     let new_id = service::read_one_model_auto_shutter_entity_cache(data, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(new_id)
// }
