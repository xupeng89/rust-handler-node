use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_config", comment = "模型动态配置表")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "property_params", column_type = "Text", default = "")]
    pub property_params: String,

    #[sea_orm(column_name = "control_params", column_type = "Text", default = "")]
    pub control_params: String,

    #[sea_orm(column_name = "rate_params", column_type = "Text", default = "")]
    pub rate_params: String,

    #[sea_orm(column_name = "flash_params", column_type = "Text", default = "")]
    pub flash_params: String,

    #[sea_orm(column_name = "model_id")]
    pub model_id: String,

    #[sea_orm(column_name = "model_state", default = 0)]
    pub model_state: i32,

    #[sea_orm(
        column_name = "filter_label_params",
        column_type = "Text",
        default = ""
    )]
    pub filter_label_params: String,

    #[sea_orm(column_name = "range_status", default = 1)]
    pub range_status: i32,

    #[sea_orm(column_name = "show_label_params", column_type = "Text", default = "")]
    pub show_label_params: String,

    #[sea_orm(column_name = "auto_shutter_params", column_type = "Text")]
    pub auto_shutter_params: String,

    #[sea_orm(column_name = "oil_params", column_type = "Text")]
    pub oil_params: String,

    #[sea_orm(column_name = "relate_point", default = "1")]
    pub relate_point: String,

    #[sea_orm(column_name = "relate_interlock", default = "1")]
    pub relate_interlock: String,

    #[sea_orm(column_name = "default_params", column_type = "Text")]
    pub default_params: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
