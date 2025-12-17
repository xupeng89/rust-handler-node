use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// ConfFunctionPic 实体定义
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "conf_function_pic_entity")] // 对应 @Entity({ name: "conf_function_pic" })
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

    /// 图片数据，base64
    /// 对应 TypeORM 的 picture: string @Column(type: "text")
    #[sea_orm(column_name = "picture", column_type = "Text")]
    pub picture: String,

    /// 唯一ID
    /// 对应 TypeORM 的 code: string @Column(type: "varchar")
    #[sea_orm(column_name = "code")] // 增加 unique 约束
    pub code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 为实体模型实现 ActiveModelBehavior
impl ActiveModelBehavior for ActiveModel {}
