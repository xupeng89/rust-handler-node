use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 物性方法- 方法表
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "pp_calc_function_property_entity",
    comment = "物性方法- 方法表"
)]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 名字
    #[sea_orm(column_name = "name")]
    pub name: String,

    /// 方法代号
    #[sea_orm(column_name = "code")]
    pub code: String,

    /// 参数名称,逗号分隔
    #[sea_orm(column_name = "args_json")]
    pub args_json: String,

    /// 是否显示, 0: 否, 1: 是
    #[sea_orm(column_name = "is_show", default = "1")]
    pub is_show: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
