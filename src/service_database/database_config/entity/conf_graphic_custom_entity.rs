use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// ConfFunctionPic 实体定义
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_graphic_custom")] // 对应 @Entity({ name: "conf_graphic_custom" })
pub struct Model {
    /**
     * 主键
     * 对应 TypeORM 的 @PrimaryGeneratedColumn
     */
    #[sea_orm(primary_key)]
    pub id: i32,

    /// 公式名称
    /// 对应 TypeORM 的 name: string @Column(type: "varchar")
    #[sea_orm(column_name = "name")]
    // String(None) 对应无长度限制的 String/Varchar
    pub name: String,

    pub code: String,

    #[sea_orm(column_name = "custom_type")]
    pub custom_type: String,

    pub arithmetic: String,

    pub size: String,
    #[sea_orm(column_name = "svg", column_type = "Text")]
    pub svg: String,

    pub ports: String,

    #[sea_orm(column_name = "window_size")]
    pub window_size: String,

    #[sea_orm(column_name = "dnd_type")]
    pub dnd_type: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
