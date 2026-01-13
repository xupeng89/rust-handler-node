use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::util_handle::model_initialize_data_in_cold_state_service::*;

paste! {
    generate_napi_methods! {
        "modelColdState",

                get_model_cold_state_list_api(model_id: String) -> Vec<ModelInitializeDataInColdStateDTO> => get_list_by_model_id,

                get_model_cold_state_default_api(model_id: String) -> Option<ModelInitializeDataInColdStateDTO> => get_default_by_model_id,

                get_model_one_by_id_model_id_api(id: i32, model_id: String) -> Option<ModelInitializeDataInColdStateDTO> => get_one_by_id_model_id,

                create_model_cold_state_api(data: ModelInitializeDataInColdStateDTO) -> i32 => create,

                set_model_others_not_default_api(model_id: String) -> bool => set_others_not_default,

                update_model_cold_state_api(data: ModelColdStateUpdateDTO) -> bool => update,

                update_model_is_default_by_model_id_api(model_id: String, is_default: i32) -> bool => update_is_default_by_model_id,

                delete_model_cold_state_api(id: i32, model_id: String) -> bool => delete,
    }
}
