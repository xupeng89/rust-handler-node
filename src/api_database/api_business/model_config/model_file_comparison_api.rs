use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::service_database::database_business::service::model_config::model_file_comparison_service::*;
use crate::{generate_napi_methods};

paste! {
    generate_napi_methods! {
        "modelFile",

                get_model_file_code_by_name_api(file_name: String) -> String => ModelFileComparisonService::get_code_by_file_name,

                insert_or_update_file_comparison_api(code: String, file_name: String) -> () => ModelFileComparisonService::insert_or_update,

                delete_file_comparison_by_name_api(file_name: String) -> () => ModelFileComparisonService::delete_by_file_name,
    }
}
