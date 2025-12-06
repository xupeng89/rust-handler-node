use crate::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{
    prelude::Expr, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult,
    IntoActiveModel, QueryFilter, QuerySelect, Set,
};

use serde::{Deserialize, Serialize};
// ======================================
use crate::database_config::entity::conf_config_entity::{
    ActiveModel as ConfConfigActiveModel,
    Column as ConfConfigColumn, // 引入 Column 枚举用于查询
    Entity as ConfConfigEntity,
    Model as ConfConfigModel,
};

// 针对 NAPI 调用的 DTO (Data Transfer Object)
// 字段与 Model 一致，但添加 napi(object) 宏
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDto {
    pub id: i32,
    pub property_params: String,
    pub control_params: String,
    pub rate_params: String,
    pub flash_params: String,
    pub filter_label_params: String,
    pub model_state: i32,
    pub show_label_params: String,
    pub range_status: i32,
    pub auto_shutter_params: String,
    pub oil_params: String,
}

// 辅助函数：将 Model 转换为 ConfigDto
impl From<ConfConfigModel> for ConfigDto {
    fn from(model: ConfConfigModel) -> Self {
        ConfigDto {
            id: model.id,
            property_params: model.property_params,
            control_params: model.control_params,
            rate_params: model.rate_params,
            flash_params: model.flash_params,
            filter_label_params: model.filter_label_params,
            model_state: model.model_state,
            show_label_params: model.show_label_params,
            range_status: model.range_status,
            auto_shutter_params: model.auto_shutter_params,
            oil_params: model.oil_params,
        }
    }
}
