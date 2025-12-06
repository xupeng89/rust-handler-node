use crate::api_cache::err_handle::handle_db_err;
use crate::database_cache::service::variable_curve_service::*;
use napi::*;
use napi_derive::napi;

#[napi(namespace = "variableCurveHandle")]
/// 插入新的变量曲线信息
pub async fn add_variable_curve_log(
    datasets: Vec<DatasetItem>,
    model_id: String,
    config_id: String,
    sim_time: f64,
) -> Result<i32> {
    // 调用服务层方法并处理错误
    let result = insert_model_variable_curve_entity_cache(datasets, model_id, config_id, sim_time)
        .await
        .map_err(handle_db_err)?;

    // SeaORM insert 返回 InsertResult，其中包含 last_insert_id
    Ok(result.last_insert_id)
}

// 注意：由于 NAPI 只能返回 JS 支持的类型 (如 Vec<JsObject>),
#[napi(namespace = "variableCurveHandle")]
/// 根据 modelId 和 configId 获取所有曲线数据，按时间排序
pub async fn list_variable_curves_by_time(
    model_id: String,
    config_id: String,
) -> Result<Vec<CurveModel>> {
    let result = get_model_variable_curve_entity_cache_by_filter_time(model_id, config_id)
        .await
        .map_err(handle_db_err)?;

    Ok(result)
}

#[napi(namespace = "variableCurveHandle")]
/// 根据 modelId, configId 和时间间隔 filterCount 获取过滤后的曲线数据
pub async fn list_variable_curves_by_filter_count(
    model_id: String,
    config_id: String,
    filter_count: i32,
) -> Result<Vec<CurveModel>> {
    let result =
        get_model_variable_curve_entity_cache_by_filter_count(model_id, config_id, filter_count)
            .await
            .map_err(handle_db_err)?;

    Ok(result)
}
