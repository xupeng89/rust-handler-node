use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::util_handle::model_trend_chart_config_service::*;

paste! {
    generate_napi_methods! {
        "modelTrendChart",

        insert_model_trend_chart_config_api(data: ModelTrendChartConfigDTO) -> bool => insert,

        get_model_trend_chart_config_count_api(model_id: String, name: String) -> i64 => get_count_by_name,

        get_many_model_trend_chart_config_api(model_id: String) -> Vec<ModelTrendChartConfigDTO> => get_many_by_model_id,

        delete_model_trend_chart_config_api(model_id: String, ids: Vec<i32>) -> bool => delete_by_ids,
    }
}
