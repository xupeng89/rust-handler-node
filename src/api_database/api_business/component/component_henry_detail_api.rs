use crate::error_handle::err_handle::*;
use crate::service_database::database_business::service::component::component_henry_detail_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn insert_compound_henry_detail_list_api(
    list: Vec<ModelComponentHenryDetailDTO>,
) -> Result<u32> {
    let res = insert_list(list).await.map_err(handle_db_err)?;
    Ok(res as u32)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn delete_compound_henry_detail_by_casno_api(
    casnos: Vec<String>,
    henry_id: String,
) -> Result<u32> {
    let res = delete_by_casno(casnos, henry_id)
        .await
        .map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn delete_compound_henry_detail_by_ids_api(ids: Vec<String>) -> Result<u32> {
    let res = delete_by_ids_cascade(ids).await.map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn delete_compound_henry_detail_by_henry_ids_api(henry_ids: Vec<String>) -> Result<u32> {
    let res = delete_by_compound_henry_ids(henry_ids)
        .await
        .map_err(handle_db_err)?;
    Ok(res)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn select_compound_henry_detail_by_henry_id_api(
    henry_id: String,
    is_default: i32,
) -> Result<Vec<ModelComponentHenryDetailDTO>> {
    select_by_henry_id(henry_id, is_default)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn select_compound_henry_detail_by_henry_ids_api(
    henry_ids: Vec<String>,
    is_default: i32,
) -> Result<Vec<ModelComponentHenryDetailDTO>> {
    select_by_henry_ids(henry_ids, is_default)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn update_compound_henry_detail_api(
    data: ModelComponentHenryDetailDTO,
) -> Result<String> {
    update_detail(data).await.map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn select_compound_henry_detail_by_ids_api(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentHenryDetailDTO>> {
    select_by_ids(ids).await.map_err(handle_db_err)
}

#[napi(namespace = "modelComponentHenryDetail")]
pub async fn select_compound_henry_detail_by_only_henry_ids_api(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentHenryDetailDTO>> {
    select_by_only_henry_ids(ids).await.map_err(handle_db_err)
}
