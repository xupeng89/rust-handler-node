use napi::Result;
use napi_derive::napi;
use serde_json::Value;

// 引入 DTO 和 Service 函数
use crate::service_database::database_physical_property::service::physical_component_base_service::*;
use crate::service_database::database_physical_property::service::physical_component_information_service::*;
use crate::service_database::database_physical_property::service::physical_component_temperature_equation_service::*;

// 引入你项目统一的错误转换函数
use crate::error_handle::err_handle::handle_db_err;

#[napi(namespace = "physicalComponent")]
/// 根据 ID 列表批量获取组分基础物理性质
pub async fn get_physical_base_by_ids_api(ids: Vec<i32>) -> Result<Vec<PhysicalBaseDTO>> {
    get_physical_base_by_ids(ids).await.map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 根据组分 ID (compound_id) 读取相关记录
pub async fn get_physical_base_by_compound_id_api(
    compound_id: i32,
) -> Result<Vec<PhysicalBaseDTO>> {
    get_physical_base_by_compound_id(compound_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 快速初始化组分基础性质数据 (全量删除并重新分段插入)
pub async fn init_physical_base_data_fast_api(init_json_data: Vec<Value>) -> Result<()> {
    init_physical_base_data_fast(init_json_data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

#[napi(namespace = "physicalComponent")]
/// 根据 casNo 获取单条组分详细信息
pub async fn get_physical_information_one_by_cas_no_api(
    cas_no: String,
) -> Result<PhysicalInformationDTO> {
    get_physical_information_one_by_cas_no(cas_no)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 根据 casNo 列表批量获取组分详细信息
pub async fn get_physical_information_list_by_cas_no_list_api(
    cas_no_list: Vec<String>,
) -> Result<Vec<PhysicalInformationDTO>> {
    get_physical_information_list_by_cas_no_list(cas_no_list)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 获取库中所有的组分信息列表
pub async fn get_all_physical_information_list_api() -> Result<Vec<PhysicalInformationDTO>> {
    get_all_physical_information_list()
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 快速初始化组分信息数据 (全量清空并分段插入)
pub async fn init_physical_information_data_fast_api(init_json_data: Vec<Value>) -> Result<()> {
    init_physical_information_data_fast(init_json_data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

#[napi(namespace = "physicalComponent")]
/// 根据 ID 列表获取组分温度关联方程记录
pub async fn get_physical_temperature_equation_by_ids_api(
    ids: Vec<i32>,
) -> Result<Vec<TemperatureEquationDTO>> {
    get_physical_temperature_equation_by_ids(ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 根据组分 ID (compound_id) 获取其所有的温度关联方程
pub async fn get_physical_temperature_equation_by_compound_id_api(
    compound_id: i32,
) -> Result<Vec<TemperatureEquationDTO>> {
    get_physical_temperature_equation_by_compound_id(compound_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "physicalComponent")]
/// 快速初始化温度关联方程数据 (全量物理删除并分段插入)
pub async fn init_physical_temperature_equation_data_fast_api(
    init_json_data: Vec<Value>,
) -> Result<()> {
    init_physical_temperature_equation_data_fast(init_json_data)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}
