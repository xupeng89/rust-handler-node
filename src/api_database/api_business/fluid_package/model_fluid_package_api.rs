use napi::Result;
use napi_derive::napi;

use crate::error_handle::err_handle::*;

use crate::service_database::database_business::service::fluid_package::model_fluid_package_service::*;

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
pub async fn update_fluid_package_api(data: ModelFluidPackageUpdateDTO) -> Result<String> {
    update_fluid_package(data).await.map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn set_fluid_package_default_api(model_id: String, target_id: String) -> Result<()> {
    set_fluid_package_default(model_id, target_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelFluidPackage")]
pub async fn get_fluid_package_count_api(model_id: String, only_default: bool) -> Result<u32> {
    let count = get_fluid_package_count(model_id, only_default)
        .await
        .map_err(handle_db_err)?;
    Ok(count as u32)
}
