use crate::error_handle::err_handle::handle_db_err;
use crate::service_database::database_business::service::model_config::model_name_generator_service::*;
use napi::Result;
use napi_derive::napi;

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

#[napi(namespace = "nameGenerator")]
pub fn get_number_segment_from_name_tool(name: String) -> i32 {
    // 纯工具函数，无需 async
    let re = regex::Regex::new(r"\d+").unwrap();
    re.find(&name)
        .map(|m| m.as_str().parse().unwrap_or(-1))
        .unwrap_or(-1)
}
