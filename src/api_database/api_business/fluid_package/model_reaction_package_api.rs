use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::service_database::database_business::service::fluid_package::model_reaction_package_service::*;
use crate::{generate_napi_methods};

paste! {
    generate_napi_methods! {
        "modelReaction",

        // 1. 查询类
        get_model_reaction_by_id_api(id: String) -> Option<ModelReactionPackageDTO> => get_model_reaction_by_id,

        get_model_reactions_by_ids_and_model_id_api(ids: Vec<String>, model_id: String) ->  Vec<ModelReactionPackageDTO> => get_model_reactions_by_ids_and_model_id,

        get_model_reactions_by_channel_id_api(channel_id: String) -> Vec<ModelReactionPackageDTO> => get_model_reactions_by_channel_id,

        get_model_reactions_by_model_id_api(model_id: String) -> Vec<ModelReactionPackageDTO> => get_model_reactions_by_model_id,

        get_model_reaction_by_name_like_api(name: String, model_id: String) -> Vec<ModelReactionPackageDTO> => get_model_reaction_by_name_like,

        // 2. 写入/更新类
        insert_model_reaction_list_api(datas: Vec<ModelReactionPackageDTO>) -> bool => insert_model_reaction_list,

        update_model_reaction_api(data: ModelReactionPackageUpdateDTO) -> bool => update_model_reaction,

        // 3. 删除类
        delete_model_reactions_by_ids_api(ids: Vec<String>) -> bool => delete_model_reactions_by_ids,
    }
}
