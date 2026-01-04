use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_oil_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentOil")]
pub async fn insert_model_component_oil_api(list: Vec<ModelComponentOilDTO>) -> Result<u32> {
    let res = insert_model_component_oil(list)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentOil")]
pub async fn get_model_component_oil_by_id_api(id: String) -> Result<Option<ModelComponentOilDTO>> {
    get_model_component_oil_by_id(id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn get_oils_by_channel_id_api(channel_id: String) -> Result<Vec<ModelComponentOilDTO>> {
    get_oils_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn get_oils_by_channel_ids_api(
    channel_ids: Vec<String>,
) -> Result<Vec<ModelComponentOilDTO>> {
    get_oils_by_channel_ids(channel_ids)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn get_oils_by_component_cas_no_api(
    cas_no: Vec<String>,
) -> Result<Vec<ModelComponentOilDTO>> {
    get_oils_by_component_cas_nos(cas_no)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn update_model_component_oil_api(data: ModelComponentOilUpdateDTO) -> Result<String> {
    update_model_component_oil(data)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn update_model_compound_oils_api(
    list: Vec<ModelComponentOilUpdateDTO>,
) -> Result<Vec<String>> {
    update_model_component_oils(list)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentOil")]
pub async fn delete_model_component_oils_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let res = delete_oils_by_ids(ids).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentOil")]
pub async fn delete_model_component_oils_by_channel_ids_api(
    channel_ids: Vec<String>,
) -> Result<u32> {
    let res = delete_oils_by_channel_ids(channel_ids)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}
