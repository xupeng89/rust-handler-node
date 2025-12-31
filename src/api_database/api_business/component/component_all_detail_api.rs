use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_all_detail_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentDetail")]
pub async fn get_one_all_compound_detail_by_id_api(
    id: String,
) -> Result<Option<ModelComponentAllDetailDTO>> {
    get_one_all_compound_detail_by_id(id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_all_model_compound_channel_count_api(channel_id: String) -> Result<u32> {
    let count = get_all_model_compound_channel_count(channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(count as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_all_detail_by_channel_id_api(
    channel_id: String,
) -> Result<Vec<ModelComponentAllDetailDTO>> {
    let result = get_all_detail_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_normal_detail_api(channel_id: String) -> Result<Vec<ComponentNormalDTO>> {
    get_normal_detail_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_normal_detail_name_casno_api(
    channel_id: String,
) -> Result<Vec<ComponentCasNoNameDTO>> {
    get_normal_detail_name_casno_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn insert_model_compound_all_detail_list_api(
    list: Vec<ModelComponentAllDetailDTO>,
) -> Result<u32> {
    let res = insert_list(list).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn delete_compound_all_detail_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let res = delete_by_ids(ids).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn update_physical_data_api(id: String, base: String, temp: String) -> Result<u32> {
    let res = update_physical_data(id, base, temp)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}
