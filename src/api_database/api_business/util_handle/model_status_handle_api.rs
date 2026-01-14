use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::util_handle::model_status_handle_service::*;
paste! {
    generate_napi_methods! {
        "modelStatus",
        get_model_params_by_material_list_api(model_id: String, node_type: String) -> Vec<serde_json::Value>  => get_params_by_material_list,

        get_model_params_by_code_list_api(graphic_id: String,model_id: String,code: String) -> Option<serde_json::Value> => get_params_by_code_list,

        get_model_all_code_params_by_list_api(graphic_id: String,model_id: String) -> Vec<serde_json::Value> => get_all_code_params_by_list,

        get_model_actived_params_by_code_api(code: String,actived: i32, model_id: String) -> Vec<serde_json::Value>  => get_actived_params_by_code,

        get_model_dynamic_info_by_code_api(code: String, model_id: String) -> Option<serde_json::Value> => get_dynamic_info_with_objects,

        update_model_node_name_api(model_id: String,graphic_id: String, name: String ) -> u32 => update_node_name,

        delete_model_params_by_graphic_id_api(model_id: String, graphic_id: String ) -> u32 => delete_params_by_graphic_id,

        delete_model_params_where_graphic_id_null_api(model_id: String) -> u32 => delete_params_where_graphic_id_null,

        add_model_node_to_all_status_versions_api(model_id: String,data: ModelStatusParamsDTO) -> bool => add_node_to_all_status_versions,

        batch_model_sync_status_params_api(datas: Vec<ModelStatusParamsDTO>, code: String, model_id: String) -> bool => batch_sync_params,

        update_or_creat_status_by_infor_and_params_api( data: ModelStatusInformationDTO, element_list: Vec<ModelStatusParamsDTO>) -> bool => update_or_creat_status_by_infor_and_params,

        update_model_active_status_bulk_api(datas: Vec<serde_json::Value>, codes: Vec<String>, model_id: String) -> bool => update_active_status_bulk,

        update_model_dynamic_objects_ex_api(model_id: String,code: String,res_graphic_element_list: Vec<serde_json::Value>) -> bool => update_dynamic_objects_ex,

        get_model_params_by_ids_and_code_api(graphic_ids: Vec<String>,model_id: String,code: Option<String>) -> Vec<serde_json::Value> => get_params_by_ids_and_code,

        update_model_info_name_api(model_id: String, code: String, name: String) -> u32 => update_info_name,

        get_model_latest_status_api(model_id: String) -> Option<ModelStatusInformationDTO> => get_latest_status,

        update_model_info_update_at_api(model_id: String, code: String) -> u32 => update_info_update_at,

        delete_model_status_cascade_api(codes: Vec<String>, model_id: String) -> bool => delete_status_cascade,

        update_model_all_graphic_model_api(update_data: Vec<serde_json::Value>) -> bool => update_all_graphic_model,

        get_model_params_by_graphic_id_all_versions_api(graphic_id: String,model_id: String) -> Vec<ModelStatusParamsDTO> => get_params_by_graphic_id_all_versions,

        get_model_status_params_by_ids_and_all_code_api(graphic_ids: Vec<String>, r#type: String,model_id: String) -> Vec<serde_json::Value> => get_status_params_by_ids_and_all_code,

        insert_model_all_params_redo_api(ntities: Vec<serde_json::Value>) -> () => insert_all_params_redo,

        update_model_params_by_type_only_init_params_api(update_datas: Vec<serde_json::Value>, r#type: String, model_id: String) -> bool => update_params_by_type_only_init_params


    }
}
