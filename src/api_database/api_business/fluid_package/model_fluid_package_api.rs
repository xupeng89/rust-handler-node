use crate::error_handle::err_handle::*;
use napi::Result;
use napi_derive::napi;
use paste::paste;

use crate::service_database::database_business::service::fluid_package::model_fluid_package_service::*;
use crate::{generate_napi_methods,generate_napi_u32_methods};

paste! {
    generate_napi_methods! {
        "modelFluidPackage",

        // 格式：函数名(参数: 类型) -> 返回值类型 => Service层对应函数名

        // 计算函数相关

        get_calc_functions_by_package_id_api(package_id: String) -> Vec<ModelPhysicalPropertyCalcDTO> => get_calc_functions_by_package_id,
        get_calc_functions_by_package_ids_api(package_ids: Vec<String>) -> Vec<ModelPhysicalPropertyCalcDTO> => get_calc_functions_by_package_ids,
        update_calc_functions_selected_api(package_id: String, list: Vec<PpMethodFunctionDTO>) -> bool => update_calc_functions_selected,
        delete_calc_functions_by_fluid_package_id_api(package_id: String) -> bool => delete_calc_functions_by_package_id,
        delete_calc_functions_by_fluid_package_ids_api(package_ids: Vec<String>) -> bool => delete_calc_functions_by_package_ids,
        insert_calc_functions_api(calc_func_list: Vec<ModelPhysicalPropertyCalcDTO>) -> bool => insert_fluid_package_calc_function,

        // 物性包基本操作
        update_fluid_package_api(data: ModelFluidPackageUpdateDTO) -> String => update_fluid_package,
        get_fluid_package_by_channel_ids_api(channel_ids: Vec<String>) -> Vec<ModelFluidPackageDTO> => get_fluid_package_by_channel_ids,
        get_fluid_package_by_id_api(package_id: String) -> Option<ModelFluidPackageDTO> => get_fluid_package_by_id,
        get_fluid_package_by_ids_api(package_ids: Vec<String>) -> Vec<ModelFluidPackageDTO> => get_fluid_package_by_ids,

        // 统计与过滤
        get_fluid_package_by_ids_and_default_flag_count_api(package_ids: Vec<String>, is_default: u32) -> u32 => get_fluid_package_by_ids_and_default_flag_count,
        get_fluid_package_model_id_default_api(model_id: String, only_default: u32) -> Option<ModelFluidPackageDTO> => get_fluid_package_by_model_id_and_default_flag,
        get_fluid_package_model_id_api(model_id: String) -> Vec<ModelFluidPackageDTO> => get_fluid_package_by_model_id_flag,
        get_fluid_package_model_id_count_api(model_id: String) -> u32 => get_fluid_package_by_model_id_count_flag,

        // 插入与默认值设置
        insert_fluid_package_api(fluid_package_list: Vec<ModelFluidPackageDTO>) -> bool => insert_fluid_package,
        set_fluid_package_default_api(model_id: String, target_id: String) -> () => set_fluid_package_default,

        // 名称搜索相关
        get_fluid_package_model_id_and_name_api(name: String, model_id: String) -> Option<ModelFluidPackageDTO> => get_fluid_package_model_id_and_name,
        get_fluid_package_model_id_and_like_name_api(name: String, model_id: String) -> Vec<ModelFluidPackageDTO> => get_fluid_package_model_id_and_like_name,
        delete_fluid_package_by_ids_api(package_ids: Vec<String>) -> bool => delete_fluid_package_by_ids,
   }
}

paste! {
    generate_napi_u32_methods! {
        "modelFluidPackage",
        get_fluid_package_model_id_default_count_api(
            model_id: String,
            only_default: u32) -> Result<u32> =>get_fluid_package_model_id_default_count
    }
}
