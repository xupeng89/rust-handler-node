use napi::Result;
use napi_derive::napi;

use crate::error_handle::err_handle::*;

use crate::service_database::database_business::service::model_config::model_config_service::*;

#[napi(namespace = "modelConfig")]
pub async fn get_model_config_by_model_id_api(model_id: String) -> Result<ModelConfigDTO> {
    let result = get_model_config_detail_by_model_id(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn update_model_config_by_model_id_api(
    model_id: String,
    data: ModelConfigUpdateDTO,
) -> Result<String> {
    let result = update_model_config_detail_by_model_id(model_id, data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn insert_model_config_api(data: ModelConfigDTO) -> Result<i32> {
    let result = insert_model_config_detail(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn get_auto_shutter_config_api(model_id: String) -> Result<AutoShutterParams> {
    let result = get_model_config_auto_shutter_config(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn get_show_label_params_config_api(model_id: String) -> Result<ShowLabelParams> {
    let result = get_model_config_show_label_params(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn get_model_config_filter_label_params_api(
    model_id: String,
) -> Result<FilterLabelParamsResult> {
    let result = get_model_config_for_filter_label_params(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelConfig")]
pub async fn get_model_config_control_and_rate_api(
    model_id: String,
) -> Result<ControlAndRateResult> {
    let result = get_model_config_control_and_rate_params(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}
