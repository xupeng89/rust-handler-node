use crate::impl_binary_syncable;
use crate::service_database::interface_trait::{HasId, SyncableBinaryEntity};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
/// 二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -nrtl
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(
    table_name = "pp_binary_nrtl_entity",
    comment = "二元交互参数信息表， 该信息表主要用于存储二元交互参数数据，具体表结构是由二元交互参数方法表去定义 -nrtl"
)]
#[serde(rename_all = "camelCase")]
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
    #[sea_orm(column_name = "aij", default = "0")] // 改为小写 aij（原 AIJ）
    pub aij: String,

    /// 参数
    #[sea_orm(column_name = "aji", default = "0")] // 改为小写 aji（原 AJI）
    pub aji: String,

    /// 参数
    #[sea_orm(column_name = "bij", default = "0")] // 改为小写 bij（原 BIJ）
    pub bij: String,

    /// 参数
    #[sea_orm(column_name = "bji", default = "0")] // 改为小写 bji（原 BJI）
    pub bji: String,

    /// 参数
    #[sea_orm(column_name = "cij", default = "0")] // 改为小写 cij（原 CIJ）
    pub cij: String,

    /// 参数
    #[sea_orm(column_name = "dij", default = "0")] // 改为小写 dij（原 DIJ）
    pub dij: String,

    /// 参数
    #[sea_orm(column_name = "eij", default = "0")] // 改为小写 eij（原 EIJ）
    pub eij: String,

    /// 参数
    #[sea_orm(column_name = "eji", default = "0")] // 改为小写 eji（原 EJI）
    pub eji: String,

    /// 参数
    #[sea_orm(column_name = "fij", default = "0")] // 改为小写 fij（原 FIJ）
    pub fij: String,

    /// 参数
    #[sea_orm(column_name = "fji", default = "0")] // 改为小写 fji（原 FJI）
    pub fji: String,

    /// 参数
    #[sea_orm(column_name = "min_t", default = "0")] // 已为小写，无需修改
    pub min_t: String,

    /// 参数
    #[sea_orm(column_name = "max_t", default = "0")] // 已为小写，无需修改
    pub max_t: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl_binary_syncable!(Model, ActiveModel, Entity);
