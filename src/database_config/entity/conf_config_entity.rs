use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_config")] // 对应 @Entity({ name: "conf_config" })
pub struct Model {
    // 对应 TypeORM 的 id: number @PrimaryGeneratedColumn
    // SeaORM 使用 auto_increment 来模拟 PrimaryGeneratedColumn
    #[sea_orm(primary_key)]
    pub id: i32,

    // 对应 TypeORM 的 propertyParams: string @Column(name: "property_params", type: "text", default: "")
    #[sea_orm(column_name = "property_params", column_type = "Text", default = "")]
    pub property_params: String,

    // 对应 TypeORM 的 controlParams: string @Column(name: "control_params", type: "text", default: "")
    #[sea_orm(column_name = "control_params", column_type = "Text", default = "")]
    pub control_params: String,

    // 对应 TypeORM 的 rateParams: string @Column(name: "rate_params", type: "text", default: "")
    #[sea_orm(column_name = "rate_params", column_type = "Text", default = "")]
    pub rate_params: String,

    // 对应 TypeORM 的 flashParams: string @Column(name: "flash_params", type: "text", default = "")
    #[sea_orm(column_name = "flash_params", column_type = "Text", default = "")]
    pub flash_params: String,

    // 对应 TypeORM 的 filterLabelParams: string @Column(name: "filter_label_params", type: "text", default: "")
    #[sea_orm(
        column_name = "filter_label_params",
        column_type = "Text",
        default = ""
    )]
    pub filter_label_params: String,

    // 对应 TypeORM 的 modelState: number @Column(name: "model_state", type: "int", default: 0)
    // TypeORM 的 number 默认为 i32，并设置 default = 0
    #[sea_orm(column_name = "model_state", default = 0)]
    pub model_state: i32,

    // 对应 TypeORM 的 showLabelParams: string @Column(name: "show_label_params", type: "text", default: "")
    #[sea_orm(
        column_name = "show_label_params",
        column_type = "Text",
        default = "{\"globalLabel\":[],\"flowSheetLabel\":[]}"
    )]
    pub show_label_params: String,

    // 对应 TypeORM 的 rangeStatus: number @Column(name: "range_status", type: "int", default = 1)
    #[sea_orm(column_name = "range_status", default = 1)]
    pub range_status: i32,

    // 对应 TypeORM 的 autoShutterParams: string @Column(name: "auto_shutter_params", type: "text", default: JSON.stringify(...))
    #[sea_orm(
        column_name = "auto_shutter_params",
        column_type = "Text",
        default = "{\"autoShutter\":1,\"autoTimeInterval\":30,\"autoCount\":60}"
    )]
    pub auto_shutter_params: String,

    // 对应 TypeORM 的 oilParams: string @Column(name: "oil_params", type: "text", default: JSON.stringify([]))
    #[sea_orm(column_name = "oil_params", column_type = "Text", default = "[]")]
    pub oil_params: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
