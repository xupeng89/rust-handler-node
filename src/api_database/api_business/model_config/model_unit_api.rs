use crate::error_handle::err_handle::handle_db_err;
use crate::service_database::database_business::service::model_config::model_unit_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_with_items_api(id: String) -> Result<UnitFullDataDTO> {
    get_model_unit_set_with_items(id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_one_by_id_api(id: String) -> Result<ModelUnitSetDTO> {
    get_model_unit_set_one_by_id(id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_all_by_model_id_api(
    model_id: String,
) -> Result<Vec<ModelUnitSetDTO>> {
    get_model_unit_set_all_by_model_id(model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_one_by_model_id_and_default_api(
    model_id: String,
) -> Result<ModelUnitSetDTO> {
    get_model_unit_set_one_by_model_id_and_default(model_id)
        .await
        .map_err(handle_db_err)
}

/// 增量更新单位集合信息（含默认值排他性逻辑）
#[napi(namespace = "modelUnit")]
pub async fn update_model_unit_set_logic_api(data: ModelUnitSetUpdateDTO) -> Result<bool> {
    update_unit_set_logic(data).await.map_err(handle_db_err)
}

/// 插入完整的单位集合及所有 Items
#[napi(namespace = "modelUnit")]
pub async fn insert_model_unit_model_full_api(
    set_data: ModelUnitSetDTO,
    items: Vec<ModelUnitItemDTO>,
) -> Result<String> {
    insert_unit_model_full(set_data, items)
        .await
        .map_err(handle_db_err)
}

/// 删除单位集及其关联项
#[napi(namespace = "modelUnit")]
pub async fn delete_all_model_unit_by_id_api(id: String) -> Result<u32> {
    delete_all_model_unit_by_id(id).await.map_err(handle_db_err)
}

/// 更新单位集基本信息（如 Code）并循环更新所有关联项的 Value
#[napi(namespace = "modelUnit")]
pub async fn update_model_unit_all_items_api(
    unit_set: ModelUnitSetItemsUpdateDTO,
    model_id: String,
) -> Result<bool> {
    update_unit_all_items(unit_set, model_id)
        .await
        .map_err(handle_db_err)
}

/// 通过set_id获取所有的item信息
#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_items_by_set_id_and_model_id_api(
    set_id: String,
    model_id: String,
) -> Result<Vec<ModelUnitItemDTO>> {
    get_model_unit_items_by_set_id_and_model_id(set_id, model_id)
        .await
        .map_err(handle_db_err)
}
#[napi(namespace = "modelUnit")]
pub async fn insert_model_unit_set_only_api(unit_set: Vec<ModelUnitSetDTO>) -> Result<bool> {
    insert_model_unit_set_only(unit_set)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_items_by_codes_and_set_id_api(
    codes: Vec<String>,
    set_id: String,
    model_id: String,
) -> Result<Vec<ModelUnitItemDTO>> {
    get_model_unit_set_items_by_codes_and_set_id(codes, set_id, model_id)
        .await
        .map_err(handle_db_err)
}

#[napi(namespace = "modelUnit")]
pub async fn get_model_unit_set_items_by_model_id_api(
    model_id: String,
) -> Result<Vec<ModelUnitItemDTO>> {
    get_model_unit_set_items_by_model_id(model_id)
        .await
        .map_err(handle_db_err)
}
