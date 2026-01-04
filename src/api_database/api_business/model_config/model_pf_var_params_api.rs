use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::model_config::model_pf_var_params_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelPfVarParams")]
pub async fn get_model_pf_var_params_by_model_id_api(
    model_id: String,
) -> Result<Vec<ModelPfVarParamsDTO>> {
    get_params_by_model_id(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelPfVarParams")]
pub async fn delete_model_pf_var_params_by_model_id_api(model_id: String) -> Result<u32> {
    let res = delete_params_by_model_id(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelPfVarParams")]
pub async fn insert_model_pf_var_params_api(list: Vec<ModelPfVarParamsDTO>) -> Result<bool> {
    let res = insert_pf_var_params(list).await.map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelPfVarParams")]
pub async fn update_model_pf_var_params_by_name_api(
    data: ModelPfVarParamsUpdateDTO,
) -> Result<u32> {
    let res = update_pf_var_params_by_name(data)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}
