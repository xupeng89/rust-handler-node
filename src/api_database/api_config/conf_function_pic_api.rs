// src/database/dcs_pointinfor_api.rs

use napi::Result;

// 引入 napi 宏和类型
use napi_derive::napi;

// 引入服务层函数和结构体
// 注意：PointData 必须是 pub 并且实现 Deserialize
use crate::service_database::database_config::service::conf_function_pic_service::*;

/// 将 sea_orm::DbErr 转换为 napi::Error，以便在 JS 中抛出异常
use crate::error_handle::err_handle::handle_db_err;

// 1. 根据类型批量查询
#[napi(namespace = "confFunctionPic")]
/// 根据 type_num 列表查询点位信息 (返回 JSON 字符串)
pub async fn get_conf_function_pic_one_message_api(code: String) -> Result<FunctionPicDto> {
    // 调用服务层函数，并处理 DbErr
    let result = select_conf_function_pic_by_code(code)
        .await
        .map_err(handle_db_err)?;

    Ok(result)
}

// 2. 批量更新或插入
#[napi(namespace = "confFunctionPic")]
/// 批量更新或插入点位信息
pub async fn update_or_insert_conf_function_pic_api(
    // napi-rs 自动将 JS 数组/对象映射到 Vec<PositionData>
    datas: Vec<NewFunctionPicDto>,
) -> Result<()> {
    // 调用服务层函数，并处理 DbErr
    upsert_and_insert_fixed_conf_pic(datas)
        .await
        .map_err(handle_db_err)?;

    // 返回 Unit 类型表示成功
    Ok(())
}
