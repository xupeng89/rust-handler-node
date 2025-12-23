use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 物性-方法-关系表
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "pp_calc_relation_property_entity",
    comment = "物性-方法-关系表"
)]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 物性连接id
    #[sea_orm(column_name = "base_physical_id")]
    pub base_physical_id: i32,

    /// 方法id
    #[sea_orm(column_name = "function_id")]
    pub function_id: i32,

    /// 默认方法id
    #[sea_orm(column_name = "default_function_id")]
    pub default_function_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
