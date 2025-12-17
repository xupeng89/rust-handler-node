use napi_derive::napi;
use sea_orm::entity::prelude::*;
use sea_orm::DeriveActiveEnum;
use serde::{Deserialize, Serialize};
// #[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
// #[sea_orm(
//     rs_type = "String",       // Rust 侧存储类型（通常为 String）
//     db_type = "String(StringLen::None)",         // 数据库侧类型（PostgreSQL 用 Enum）
//     enum_name = "code", // 数据库枚举名（需手动创建或通过迁移生成
//     rename_all = "camelCase"
// )]
// #[napi(string_enum)]
// pub enum ConfConfigCodeEnum {
//     #[sea_orm(string_value = "defaultParams")]
//     DefaultParams,
//     #[sea_orm(string_value = "controlParams")]
//     ControlParams,
//     #[sea_orm(string_value = "rateParams")]
//     RateParams,
//     #[sea_orm(string_value = "flashParams")]
//     FlashParams,
//     #[sea_orm(string_value = "flowLabelFliterConfig")]
//     FlowLabelFliterConfig, // 流程图标签过滤配置 -- 流股标签
//     #[sea_orm(string_value = "flowLabelShowConfig")]
//     FlowLabelShowConfig, // 流程图标签显示配置 -- 流股标签
//     #[sea_orm(string_value = "rangeStatus")]
//     RangeStatus, // 范围状态 -- 流股标签
//     #[sea_orm(string_value = "modelState")]
//     ModelState, // 模型状态
//     #[sea_orm(string_value = "autoShutterParams")]
//     AutoShutterParams, // 自动快门参数
//     #[sea_orm(string_value = "oilParams")]
//     OilParams, // 油参配置
// }
#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",       // Rust 侧存储类型（通常为 String）
    db_type = "String(StringLen::None)",         // 数据库侧类型（PostgreSQL 用 Enum）
    enum_name = "value_type", // 数据库枚举名（需手动创建或通过迁移生成）
    rename_all = "camelCase"
)]
#[napi(string_enum)]
pub enum ConfConfigValueTypeEnum {
    #[sea_orm(string_value = "json")]
    Json, // JSON 类型
    #[sea_orm(string_value = "string")]
    String, // 字符串类型
    #[sea_orm(string_value = "number")]
    Number, // 数值类型
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_config")] // 对应 @Entity({ name: "conf_config" })
pub struct Model {
    // 对应 TypeORM 的 id: number @PrimaryGeneratedColumn
    // SeaORM 使用 auto_increment 来模拟 PrimaryGeneratedColumn
    #[sea_orm(primary_key)]
    pub id: i32,

    // 中文描述
    #[sea_orm(column_name = "name", default = "")]
    pub name: String,

    // 配置编码
    #[sea_orm(column_name = "code", default = "")]
    pub code: String,

    #[sea_orm(column_name = "value", column_type = "Text", default = "")]
    pub value: String,

    // 值类型
    #[sea_orm(column_name = "value_type", default = "")]
    pub value_type: ConfConfigValueTypeEnum,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
