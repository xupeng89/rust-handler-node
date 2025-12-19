// src/database/dcs_pointinfor_api.rs

use napi::Result;

// 引入 napi 宏和类型
use napi_derive::napi;

// 引入服务层函数和结构体
// 注意：PointData 必须是 pub 并且实现 Deserialize
use crate::service_database::database_config::service::conf_unit_service::*;

/// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
use crate::error_handle::err_handle::handle_db_err;

// 1. 根据类型批量查询
#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_set_api() -> Result<Vec<ConfUnitSetDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_set_all().await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_one_unit_set_api(code: String) -> Result<ConfUnitSetDto> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_set_one(code)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confUnit")]
/// 批量更新或插入点位信息
pub async fn updata_and_insert_conf_unit_set_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<ConfUnitSetDto>,
) -> Result<i32> {
    // 调用服务层函数，并处理 DbErr
    let result = upsert_and_insert_conf_unit_set(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 1. 根据类型批量查询
#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_item_api(set_code: String) -> Result<Vec<ConfUnitItemDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_item_all(set_code)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_item_by_codes_api(
    set_code: String,
    codes: Vec<String>,
) -> Result<Vec<ConfUnitItemDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_item_all_by_codes(set_code, codes)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confUnit")]
/// 批量更新或插入点位信息
pub async fn updata_and_insert_conf_unit_item_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<ConfUnitItemDto>,
) -> Result<i32> {
    // 调用服务层函数，并处理 DbErr
    let result = upsert_and_insert_conf_unit_item(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_first_category_api() -> Result<Vec<ConfUnitFirstCategoryDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_first_category_all()
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confUnit")]
/// 批量更新或插入点位信息
pub async fn updata_and_insert_conf_unit_first_category_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<ConfUnitFirstCategoryDto>,
) -> Result<i32> {
    // 调用服务层函数，并处理 DbErr
    let result = upsert_and_insert_conf_unit_first_category(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_second_category_api() -> Result<Vec<ConfUnitSecondCategoryDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_second_category_all()
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confUnit")]
/// 批量更新或插入点位信息
pub async fn updata_and_insert_conf_unit_second_category_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<ConfUnitSecondCategoryDto>,
) -> Result<i32> {
    // 调用服务层函数，并处理 DbErr
    let result = upsert_and_insert_conf_unit_second_category(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "confUnit")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_all_unit_item_category_api() -> Result<Vec<ConfUnitItemCategoryDto>> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_unit_item_category_all()
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confUnit")]
/// 批量更新或插入点位信息
pub async fn updata_and_insert_conf_unit_item_category_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    data: Vec<ConfUnitItemCategoryDto>,
) -> Result<i32> {
    // 调用服务层函数，并处理 DbErr
    let result = upsert_and_insert_conf_unit_item_category(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}
