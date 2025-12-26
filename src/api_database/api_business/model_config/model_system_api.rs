use crate::error_handle::err_handle::handle_db_err;

use crate::service_database::database_business::service::model_config::model_system_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelSystem")]
pub async fn get_system_detail_api(model_id: String) -> Result<Vec<ModelSystemVariableDTO>> {
    get_system_detail_by_model_id(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelSystem")]
pub async fn update_system_detail_api(systems: Vec<ModelSystemVariableDTO>) -> Result<bool> {
    update_system_detail_batch(systems)
        .await
        .map_err(handle_db_err)?;
    Ok(true)
}

#[napi(namespace = "modelSystem")]
pub async fn init_model_system_variable_api(
    systems: Vec<ModelSystemVariableDTO>,
    model_id: String,
) -> Result<bool> {
    init_model_system_variable_detail(systems, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(true)
}
