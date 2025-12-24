use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -uniquac
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "pp_binary_uniquac_entity",
    comment = "二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -uniquac"
)]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 关联组分iid
    #[sea_orm(column_name = "component_i_id")] // 已为小写，无需修改（原本就是小写）
    pub component_i_id: i32,

    /// 关联组分i的CASNO
    #[sea_orm(column_name = "component_i")] // 已为小写，无需修改
    pub component_i: String,

    /// 关联组分jid
    #[sea_orm(column_name = "component_j_id")] // 已为小写，无需修改
    pub component_j_id: i32,

    /// 关联组分j的casno
    #[sea_orm(column_name = "component_j")] // 已为小写，无需修改
    pub component_j: String,

    /// 参数
    #[sea_orm(column_name = "aij", default = "0")] // 大写转小写列名
    pub aij: String,

    /// 参数
    #[sea_orm(column_name = "aji", default = "0")] // 大写转小写列名
    pub aji: String,

    /// 参数
    #[sea_orm(column_name = "bij", default = "0")] // 大写转小写列名
    pub bij: String,

    /// 参数
    #[sea_orm(column_name = "bji", default = "0")] // 大写转小写列名
    pub bji: String,

    /// 参数
    #[sea_orm(column_name = "cij", default = "0")] // 大写转小写列名
    pub cij: String,

    /// 参数
    #[sea_orm(column_name = "cji", default = "0")] // 大写转小写列名
    pub cji: String,

    /// 参数
    #[sea_orm(column_name = "dij", default = "0")] // 大写转小写列名
    pub dij: String,

    /// 参数
    #[sea_orm(column_name = "dji", default = "0")] // 大写转小写列名
    pub dji: String,

    /// 参数
    #[sea_orm(column_name = "eij", default = "0")] // 大写转小写列名
    pub eij: String,

    /// 参数
    #[sea_orm(column_name = "eji", default = "0")] // 大写转小写列名
    pub eji: String,

    /// 参数
    #[sea_orm(column_name = "fij", default = "0")] // 大写转小写列名
    pub fij: String,

    /// 参数
    #[sea_orm(column_name = "fji", default = "0")] // 大写转小写列名
    pub fji: String,

    /// 参数
    #[sea_orm(column_name = "min_t", default = "0")]
    pub min_t: String,

    /// 参数
    #[sea_orm(column_name = "max_t", default = "0")]
    pub max_t: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
use crate::impl_binary_syncable;
use crate::service_database::interface_trait::{HasId, SyncableBinaryEntity};
impl_binary_syncable!(Model, Entity);
