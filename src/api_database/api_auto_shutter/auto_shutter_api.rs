use napi::Result;
use napi_derive::napi;

use crate::error_handle::err_handle::*;
use crate::service_database::database_auto_shutter::service::auto_shutter_service::*;
use crate::{generate_napi_i32_methods, generate_napi_methods, generate_napi_u32_methods};
use paste::paste;

paste! {
    generate_napi_methods! {
        "autoShutter",
        get_auto_shutter_cache_list_api(
            order_flag: String, // "DESC" or "ASC"
            auto_count: u32,
            model_id: String) -> Vec<AutoShutterListItem> => get_all_model_auto_shutter_entity_list_cache,
         get_auto_shutter_cache_detail_api(id: i32, model_id: String) -> FullCacheData => get_model_auto_shutter_entity_by_id_cache
    }
}

paste! {
    generate_napi_i32_methods! {
        "autoShutter",
        update_auto_shutter_cache_api(data: AutoShutterData, model_id: String) -> i32 => update_model_auto_shutter_entity_cache,
        insert_auto_shutter_cache_api(
            data: AutoShutterData,
            model_id: String) -> i32 => read_one_model_auto_shutter_entity_cache
    }
}

paste! {
    generate_napi_u32_methods! {
        "autoShutter",
        get_all_model_auto_shutter_cache_entity_count(model_id: String) -> u32 => get_model_auto_shutter_entity_count,
    }
}

// #[napi(namespace = "autoShutter")]
// /// 更新自动快照 (Service: update_model_auto_shutter_entity_cache)
// pub async fn update_auto_shutter_cache_api(data: AutoShutterData, model_id: String) -> Result<i32> {
//     let result = update_model_auto_shutter_entity_cache(data, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutter")]
// /// 获取快照列表 (Service: get_all_model_auto_shutter_entity_list_cache)
// pub async fn get_auto_shutter_cache_list_api(
//     order_flag: String, // "DESC" or "ASC"
//     auto_count: u32,
//     model_id: String,
// ) -> Result<Vec<AutoShutterListItem>> {
//     // 注意：服务层需要 u64，这里进行转换
//     let result = get_all_model_auto_shutter_entity_list_cache(order_flag, auto_count, model_id)
//         .await
//         .map_err(handle_db_err)?;

//     Ok(result)
// }

// #[napi(namespace = "autoShutter")]
// /// 获取单个快照详情 (Service: get_model_auto_shutter_entity_by_id_cache)
// pub async fn get_auto_shutter_cache_detail_api(id: i32, model_id: String) -> Result<FullCacheData> {
//     let result = get_model_auto_shutter_entity_by_id_cache(id, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutter")]
// /// 获取目前快照数量
// pub async fn get_all_model_auto_shutter_cache_entity_count(model_id: String) -> Result<u32> {
//     let result = get_model_auto_shutter_entity_count(model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(result)
// }

// #[napi(namespace = "autoShutter")]
// ///插入新的快照记录 (Service: insert_model_auto_shutter_entity_cache)
// pub async fn insert_auto_shutter_cache_api(
//     data: AutoShutterData,
//     model_id: String,
// ) -> napi::Result<i32> {
//     let new_id = read_one_model_auto_shutter_entity_cache(data, model_id)
//         .await
//         .map_err(handle_db_err)?;
//     Ok(new_id)
// }
