use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::util_handle::model_variable_choose_service::*;

paste! {
    generate_napi_methods! {
        "modelVariable",

        insert_model_variable_choose_api(data: ModelVariableChooseDTO) -> bool => insert,

        get_model_variable_choose_by_id_api(id: String, model_id: String) -> Option<ModelVariableChooseDTO> => get_by_id_and_model,

        get_model_variable_choose_by_model_id_api(model_id: String, r#type: String) -> Vec<ModelVariableChooseDTO> => get_by_model_and_type,

        get_model_variable_choose_by_model_id_like_type_api(model_id: String, r#type: String) -> Vec<ModelVariableChooseDTO> => get_by_model_like_type,

        delete_model_variable_choose_by_id_api(id: String) -> bool => delete_by_id,

        delete_model_variable_choose_by_ids_api(ids: Vec<String>, model_id: String, r#type: String) -> bool => delete_batch,

        delete_model_variable_choose_by_types_api(types: Vec<String>, model_id: String) -> bool => delete_by_types,

        update_model_variable_choose_api(data: ModelVariableChooseUpdateDTO) -> bool => update,
    }
}
