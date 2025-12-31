use crate::error_handle::err_handle::handle_db_err;
use crate::service_database::database_business::service::model_config::model_name_generator_service::*;
use napi::Result;
use napi_derive::napi;

#[napi(namespace = "nameGenerator")]
pub async fn get_used_name_detail_api(code: String, model_id: String) -> Result<NameGenDTO> {
    let result = get_used_name_detail(code, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(result)
}

#[napi(namespace = "nameGenerator")]
pub async fn update_name_segment_by_delete_api(
    code: String,
    number_delete: String,
    model_id: String,
) -> Result<()> {
    update_number_segment_delete(code, number_delete, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

#[napi(namespace = "nameGenerator")]
pub async fn update_name_segment_by_name_number_api(
    code: String,
    number_number: String,
    model_id: String,
) -> Result<()> {
    update_number_segment_name_number(code, number_number, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}

#[napi(namespace = "nameGenerator")]
pub async fn save_or_update_name_gen_api(
    code: String,
    number_segment: String,
    model_id: String,
) -> Result<()> {
    insert_or_update_name_business(code, number_segment, model_id)
        .await
        .map_err(handle_db_err)?;
    Ok(())
}
