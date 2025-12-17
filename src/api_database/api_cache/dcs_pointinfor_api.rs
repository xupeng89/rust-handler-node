// src/database/dcs_pointinfor_api.rs

use napi::Result;

// 引入 napi 宏和类型
use napi_derive::napi;

// 引入服务层函数和结构体
// 注意：PointData 必须是 pub 并且实现 Deserialize
use crate::service_database::database_cache::service::dcs_pointinfor_service::*;

/// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
use crate::error_handle::err_handle::handle_db_err;

// 1. 根据类型批量查询
#[napi(namespace = "pointInfor")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_position_information_cache_by_types_api(
    types: Vec<i32>,
) -> Result<Vec<PositionData>> {
    // 调用服务层函数，并处理 DbErr
    let result = get_position_information_cache_by_types(types)
        .await
        .map_err(handle_db_err)?;

    Ok(result)
}

// 2. 查询所有信息
#[napi(namespace = "pointInfor")]
/// 查询所有点位信息 (返回 JSON 字符串)
pub async fn get_position_information_cache_all_message_api() -> Result<Vec<PositionData>> {
    let result = get_position_information_cache_all_message()
        .await
        .map_err(handle_db_err)?;

    Ok(result)
}

// 3. 批量更新或插入
#[napi(namespace = "pointInfor")]
/// 批量更新或插入点位信息
pub async fn update_or_insert_position_information_cache_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<PositionData>,
) -> Result<()> {
    // 调用服务层函数，并处理 DbErr
    update_or_insert_position_information_cache(data)
        .await
        .map_err(handle_db_err)?;

    // 返回 Unit 类型表示成功
    Ok(())
}
