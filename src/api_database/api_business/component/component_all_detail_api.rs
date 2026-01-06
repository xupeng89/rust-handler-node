use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_all_detail_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentDetail")]
pub async fn get_one_all_component_detail_by_id_api(
    id: String,
) -> Result<Option<ModelComponentAllDetailDTO>> {
    get_one_all_component_detail_by_id(id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_all_model_component_channel_count_api(channel_id: String) -> Result<u32> {
    let count = get_all_model_component_channel_count(channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(count as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn select_component_all_detail_have_by_name_api(
    name: String,
    channel_id: String,
) -> Result<bool> {
    let count = select_component_all_detail_have_by_name(name, channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(count)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_all_component_detail_by_channel_id_api(
    channel_id: String,
) -> Result<Vec<ModelComponentAllDetailDTO>> {
    let result = get_all_detail_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_all_detail_by_channel_id_vec_api(
    channel_id: Vec<String>,
) -> Result<Vec<ModelComponentAllDetailDTO>> {
    let result = get_all_component_detail_by_channel_id_vec(channel_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn find_component_all_detail_by_ids_api(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentAllDetailDTO>> {
    let result = find_by_ids(ids).await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn find_component_all_detail_casno_by_ids_api(ids: Vec<String>) -> Result<Vec<String>> {
    let result = find_casno_by_ids(ids).await.map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_component_normal_detail_api(
    channel_id: String,
) -> Result<Vec<ComponentNormalDTO>> {
    get_normal_detail_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn get_component_normal_detail_name_casno_api(
    channel_id: String,
) -> Result<Vec<ComponentCasNoNameDTO>> {
    get_normal_detail_name_casno_by_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn insert_model_component_all_detail_list_api(
    list: Vec<ModelComponentAllDetailDTO>,
) -> Result<u32> {
    let res = insert_list(list).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn delete_component_all_detail_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let res = delete_by_ids(ids).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn delete_component_all_detail_by_channel_ids_api(
    channel_ids: Vec<String>,
) -> Result<u32> {
    let res = delete_by_component_channel_ids(channel_ids)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn update_component_physical_data_api(
    id: String,
    base: String,
    temp: String,
) -> Result<String> {
    let res = update_physical_data(id, base, temp)
        .await
        .map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelComponentDetail")]
pub async fn update_model_component_all_detail_option_api(
    update_data: ModelComponentAllDetailUpdateDTO,
) -> Result<String> {
    let res = update_component_all_detail_data(update_data)
        .await
        .map_err(handle_db_err)?;
    Ok(res)
}
