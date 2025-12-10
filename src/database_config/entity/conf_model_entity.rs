use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// ConfFunctionPic 实体定义
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_function_pic")] // 对应 @Entity({ name: "conf_function_pic" })
pub struct Model {
    /**
     * 主键
     * 对应 TypeORM 的 @PrimaryGeneratedColumn
     */
    #[sea_orm(primary_key)]
    pub id: String,

    /// 公式名称
    #[sea_orm(column_name = "model_no")]
    pub model_no: String,

    #[sea_orm(column_name = "develop_name")]
    pub develop_name: String,

    #[sea_orm(column_name = "model_name")]
    pub model_name: String,

    #[sea_orm(column_name = "standard_temperature")]
    pub standard_temperature: i32,
    #[sea_orm(column_name = "model_name")]
    pub model_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
