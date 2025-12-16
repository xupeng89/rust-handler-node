use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 配置数据库单位集表组合
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_unit_correlation")] // 对应 @Entity({ name: "conf_pf_model_params" })
pub struct Model {
    /**
     * 主键
     * 对应 TypeORM 的 @PrimaryGeneratedColumn
     */
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "en_name")]
    pub en_name: String,

    #[sea_orm(column_name = "cn_name")]
    pub name: String,

    #[sea_orm(column_name = "code")]
    pub code: String,

    #[sea_orm(column_name = "type_item")]
    pub type_item: String,

    #[sea_orm(column_name = "unit_set_id")]
    pub unit_set_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
