use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::model_config::model_pf_model_params_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelPfModelParams")]
pub async fn get_all_pf_model_params_api(model_id: String) -> Result<Vec<ModelPfModelParamsDTO>> {
    get_all_pf_model_params_by_model_id(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelPfModelParams")]
pub async fn insert_pf_model_params_message_api(list: Vec<ModelPfModelParamsDTO>) -> Result<bool> {
    let res = insert_pf_model_params_message(list)
        .await
        .map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelPfModelParams")]
pub async fn update_pf_model_params_message_api(
    data: ModelPfModelParamsUpdateDTO,
) -> Result<String> {
    update_pf_model_params_message(data)
        .await
        .map_err(handle_db_err)
}
