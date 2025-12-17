// 组分通道信息列表
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize}; // 包含常用的 Trait 和宏
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_position_information_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,

    pub name_display: String,

    // 注意：TypeORM 中 default: "" on int 是不合理的。在 SeaORM 中，
    // 如果字段有 default 值，通常需要确保其类型匹配。假设默认值应为 0。
    pub type_num: i32, // 使用 r#type 以避免与 Rust 关键字冲突

    pub type_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
