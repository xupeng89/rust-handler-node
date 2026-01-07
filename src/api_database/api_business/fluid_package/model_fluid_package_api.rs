use napi::Result;
use napi_derive::napi;

use crate::error_handle::err_handle::*;

use crate::service_database::database_business::service::fluid_package::{
     model_fluid_package_service::*,
};

#[napi(namespace = "modelFluidPackage")]
pub async fn get_calc_functions_by_package_id_api(
    package_id: String,
) -> Result<Vec<ModelPhysicalPropertyCalcDTO>> {
    get_calc_functions_by_package_id(package_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_calc_functions_by_package_ids_api(
    package_ids: Vec<String>,
) -> Result<Vec<ModelPhysicalPropertyCalcDTO>> {
    get_calc_functions_by_package_ids(package_ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn update_calc_functions_selected_api(
    package_id: String,
    list: Vec<PpMethodFunctionDTO>,
) -> Result<bool> {
    update_calc_functions_selected(package_id, list)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn delete_calc_functions_by_fluid_package_id_api(package_id: String) -> Result<bool> {
    delete_calc_functions_by_package_id(package_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn insert_calc_functions_api(
    calc_func_list: Vec<ModelPhysicalPropertyCalcDTO>,
) -> Result<bool> {
    insert_fluid_package_calc_function(calc_func_list)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn update_fluid_package_api(data: ModelFluidPackageUpdateDTO) -> Result<String> {
    update_fluid_package(data).await.map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_by_channel_ids_api(
    channel_ids: Vec<String>,
) -> Result<Vec<ModelFluidPackageDTO>> {
    get_fluid_package_by_channel_ids(channel_ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_by_id_api(
    package_id: String,
) -> Result<Option<ModelFluidPackageDTO>> {
    get_fluid_package_by_id(package_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_by_ids_and_default_flag_count_api(
    package_ids: Vec<String>,
    is_default: u32,
) -> Result<u32> {
    get_fluid_package_by_ids_and_default_flag_count(package_ids, is_default)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_by_ids_api(
    package_ids: Vec<String>,
) -> Result<Vec<ModelFluidPackageDTO>> {
    get_fluid_package_by_ids(package_ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_model_id_default_api(
    model_id: String,
    only_default: u32,
) -> Result<Option<ModelFluidPackageDTO>> {
    get_fluid_package_by_model_id_and_default_flag(model_id, only_default)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_model_id_api(model_id: String) -> Result<Vec<ModelFluidPackageDTO>> {
    get_fluid_package_by_model_id_flag(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_model_id_count_api(model_id: String) -> Result<u32> {
    get_fluid_package_by_model_id_count_flag(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn insert_fluid_package_api(
    fluid_package_list: Vec<ModelFluidPackageDTO>,
) -> Result<bool> {
    insert_fluid_package(fluid_package_list)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn set_fluid_package_default_api(model_id: String, target_id: String) -> Result<()> {
    set_fluid_package_default(model_id, target_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_model_id_default_count_api(
    model_id: String,
    only_default: u32,
) -> Result<u32> {
    let count = get_fluid_package_model_id_default_count(model_id, only_default)
        .await
        .map_err(handle_db_err)?;
    Ok(count as u32)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_model_id_and_name_api(
    name: String,
    model_id: String,
) -> Result<bool> {
    get_fluid_package_model_id_and_name(name, model_id)
        .await
        .map_err(handle_db_err)
}
