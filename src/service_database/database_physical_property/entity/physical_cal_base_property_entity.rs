use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 物性方法- 物性系数表
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "pp_calc_base_property_entity",
    comment = "物性方法- 物性系数表"
)]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,

    /// 名字
    #[sea_orm(column_name = "name")]
    pub name: String,

    /// 方法代号
    #[sea_orm(column_name = "code")]
    pub code: String,

    /// 类型：  数组, 双精度
    #[sea_orm(column_name = "type_str")]
    pub type_str: String, // 注意：type是Rust关键字，需用r#转义

    /// 属性标识
    #[sea_orm(column_name = "key")]
    pub key: String,

    /// 气象，液相
    #[sea_orm(column_name = "phase")]
    pub phase: String,

    /// 混合物=1 纯组分=0
    #[sea_orm(column_name = "mixture")]
    pub mixture: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
