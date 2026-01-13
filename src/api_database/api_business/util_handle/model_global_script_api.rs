use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::util_handle::model_global_script_service::*;

paste! {
    generate_napi_methods! {
        "modelScript",

        get_model_script_by_id_api(id: String) -> Option<ModelGlobalScriptDTO> => get_by_id,

        get_model_scripts_by_type_api(model_id: String, type_num: i32) -> Vec<ModelGlobalScriptDTO> => get_all_by_type,

        get_model_scripts_by_ids_api(ids: Vec<String>) -> Vec<ModelGlobalScriptDTO> => get_all_by_ids,

        get_model_unit_scripts_api(model_id: String, unit_id: String) -> Vec<ModelGlobalScriptDTO> => get_unit_scripts,

        get_model_all_by_model_id_api(model_id:String) -> Vec<ModelGlobalScriptDTO> => get_all_by_model_id,

        insert_model_script_api(data: ModelGlobalScriptDTO) -> bool => insert,

        update_model_script_api(data: ModelGlobalScriptUpdateDTO) -> bool => update,

        delete_model_scripts_api(ids: Vec<String>) -> bool => delete_by_ids,
    }
}
