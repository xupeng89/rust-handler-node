use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_channel_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentChannel")]
pub async fn get_first_component_channel_api(
    id: String,
    model_id: String,
) -> Result<Option<ModelComponentChannelDTO>> {
    let result = get_first_component_channel(id, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn get_component_channel_by_model_first_api(
    model_id: String,
) -> Result<ModelComponentChannelDTO> {
    let result = get_component_channel_by_model_first(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn get_all_component_channel_by_model_id_api(
    model_id: String,
) -> Result<Vec<ModelComponentChannelDTO>> {
    let result = get_all_component_channel_by_model_id(model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn get_all_component_channel_by_ids_api(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentChannelDTO>> {
    let result = get_all_component_channel_by_ids(ids)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn insert_component_channel_api(data: ModelComponentChannelDTO) -> Result<String> {
    // 注意：TS 原有的 getCurrentName 自动重命名逻辑，建议在调用此 API 前在 Node 端计算好，
    // 或者在此处集成 Rust 版的 getCurrentName 逻辑。
    let result = insert_component_channel(data)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn insert_component_channels_copy_api(
    datas: Vec<ModelComponentChannelDTO>,
) -> Result<bool> {
    let result = insert_component_channels_copy(datas)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn delete_component_channel_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let result = delete_component_channel_by_ids(ids)
        .await
        .map_err(handle_db_err)?;
    Ok(result as u32)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn update_component_channel_api(
    data: ModelComponentChannelUpdateDTO,
    model_id: String,
) -> Result<String> {
    let result = update_component_channel(data, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn select_component_channel_by_name_count_api(
    name: String,
    model_id: String,
) -> Result<bool> {
    let result = select_component_channel_by_name(name, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "modelComponentChannel")]
pub async fn select_component_channel_by_name_like_api(
    name: String,
    model_id: String,
) -> Result<Vec<ModelComponentChannelDTO>> {
    let result = get_component_channel_by_name_like(name, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}
