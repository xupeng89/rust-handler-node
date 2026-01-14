use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::generate_napi_methods;
use crate::service_database::database_business::service::model_config::model_graphic_page_service::*;

paste! {
    generate_napi_methods! {
        "modelGraphicPage",

        get_graphic_page_by_id_api(id: String) -> Option<ModelGraphicPageDTO> => get_graphic_page_one_by_id,

        get_graphic_page_list_api(model_id: String) -> Vec<ModelGraphicPageDTO> =>get_graphic_page_list_by_model_id,

        create_graphic_page_api(data: ModelGraphicPageDTO) -> bool => create_graphic_page,

        update_graphic_page_api(data: ModelGraphicPageUpdateDTO) -> bool => update_graphic_page,

        delete_graphic_page_by_id_api(id: String) -> bool => delete_by_id_graphic_page,
    }
}
