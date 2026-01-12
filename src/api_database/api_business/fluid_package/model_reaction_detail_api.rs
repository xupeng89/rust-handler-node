use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::service_database::database_business::service::fluid_package::model_reaction_detail_service::*;
use crate::{generate_napi_methods,generate_napi_u32_methods};

paste! {
    generate_napi_methods! {
        "modelReactionDetail",
                // 详情表查询
                get_model_reaction_detail_by_id_api(id: String) -> Option<ModelReactionDetailDTO> => get_model_reaction_detail_by_id,

                get_model_reaction_details_by_reaction_id_api(reaction_id: String) -> Vec<ModelReactionDetailDTO> => get_model_reaction_details_by_reaction_id,

                get_model_reaction_details_list_by_reaction_ids_api(ids: Vec<String>) -> Vec<ModelReactionDetailDTO> => get_model_reaction_details_list_by_reaction_ids,

                // 详情表写入与修改
                insert_model_reaction_detail_api(datas: Vec<ModelReactionDetailDTO>) -> bool => insert_model_reaction_detail,

                update_model_reaction_detail_api(data: ModelReactionDetailUpdateDTO) -> bool => update_model_reaction_detail,

                // 详情表删除与校验
                delete_model_reaction_details_by_ids_api(ids: Vec<String>) -> bool => delete_model_reaction_details_by_ids,


    }
}
paste! {
    generate_napi_u32_methods! {
        "modelReactionDetail",
        check_model_reaction_package_count_api(package_name: String, model_id: String) -> u32 => check_model_reaction_package_count,
    }
}
