use napi::Result;
use napi_derive::napi;
use serde_json::Value;

// 引入 DTO 和服务层函数
use crate::service_database::database_physical_property::service::physical_calc_service::*;
// 引入错误处理
use crate::error_handle::err_handle::handle_db_err;

#[napi(namespace = "physicalCalc")]
/// 获取所有方法列表
pub async fn get_pp_calc_function_list_api() -> Result<Vec<CalcFunctionDTO>> {
    get_pp_calc_function_list().await.map_err(handle_db_err)
}

#[napi(namespace = "physicalCalc")]
/// 根据 ID 列表批量查询方法
pub async fn get_all_pp_calc_function_by_ids_api(ids: Vec<i32>) -> Result<Vec<CalcFunctionDTO>> {
    get_all_pp_calc_function_by_ids(ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalCalc")]
/// 根据 Code 列表批量查询方法
pub async fn get_first_pp_calc_function_by_codes_api(
    codes: Vec<String>,
) -> Result<Vec<CalcFunctionDTO>> {
    get_first_pp_calc_function_by_codes(codes)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalCalc")]
/// 根据 BasePhysicalId 获取 Function 选项 (用于下拉框)
pub async fn get_pp_function_options_by_bp_id_api(
    bp_id: i32,
) -> Result<Vec<crate::tool_handle::result_entity::FunctionOptionDto>> {
    get_pp_function_options_by_bp_id(bp_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalCalc")]
/// 根据方法 ID 获取流体包详细关联信息
pub async fn get_fluid_package_all_bp_by_function_id_api(
    function_id: i32,
) -> Result<Vec<CalcAllDetailDto>> {
    get_fluid_package_all_bp_by_function_id(function_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalCalc")]
/// 批量初始化/同步物性计算相关消息数据
/// data_type: "basePhysical" | "function" | "relation"
pub async fn init_pp_calc_all_msg_data_api(data_type: String, data: Vec<Value>) -> Result<()> {
    init_pp_calc_all_msg_data(&data_type, data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}
