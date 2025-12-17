use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 系统变量配置表--默认系统配置数据
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_system_variable_entity")] // 对应 @Entity({ name: "conf_system_variable_entity" })
pub struct Model {
    /**
     * 主键
     * 对应 TypeORM 的 @PrimaryGeneratedColumn
     */
    #[sea_orm(primary_key)]
    pub id: i32,

    #[sea_orm(column_name = "code")]
    pub code: String,

    #[sea_orm(column_name = "name")]
    pub name: String,

    #[sea_orm(column_name = "value")]
    pub value: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
