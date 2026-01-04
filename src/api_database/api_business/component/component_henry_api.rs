use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_henry_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentHenry")]
pub async fn insert_component_henry_list_api(list: Vec<ModelComponentHenryDTO>) -> Result<u32> {
    let res = insert_component_henry_list(list)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn select_component_henry_by_name_api(name: String, channel_id: String) -> Result<bool> {
    select_component_henry_by_name(name, channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn delete_component_henry_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let res = delete_component_henry_by_ids(ids)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn update_component_henry_name_api(id: String, name: String) -> Result<bool> {
    update_component_henry_name(id, name)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn select_by_component_channel_id_api(
    channel_id: String,
) -> Result<Vec<ModelComponentHenryDTO>> {
    select_by_component_channel_id(channel_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn henry_connect_component_detail_api(msg: ComponentHenryConnectDTO) -> Result<u32> {
    let res = henry_connect_component_detail(msg)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn delete_by_component_channel_ids_api(channel_ids: Vec<String>) -> Result<u32> {
    let res = delete_by_component_channel_ids(channel_ids)
        .await
        .map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentHenry")]
pub async fn get_model_component_henry_by_ids_api(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentHenryDTO>> {
    get_model_component_henry_by_ids(ids)
        .await
        .map_err(handle_db_err)
}
