use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::model_config::model_graphic_page_service::*;

paste! {
    generate_napi_methods! {
        "modelFlowSheet",

        get_flow_sheet_by_id_api(id: String) -> Option<ModelGraphicPageDTO> => get_one_by_id,

        get_flow_sheet_list_api(model_id: String) -> Vec<ModelGraphicPageDTO> =>get_list_by_model_id,

        create_flow_sheet_api(data: ModelGraphicPageDTO) -> bool => create,

        update_flow_sheet_api(data: ModelFlowSheetUpdateDTO) -> bool => update,

        delete_flow_sheet_by_id_api(id: String) -> bool => delete_by_id,
    }
}
