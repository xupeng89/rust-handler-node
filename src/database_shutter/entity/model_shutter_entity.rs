use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "model_shutter_entity")]
pub struct Model {
    // 对应 TypeORM 的 id: string @PrimaryColumn
    #[sea_orm(primary_key, column_name = "id")]
    pub id: String,

    // 对应 TypeORM 的 name: string @Column(name: "name", default: "")
    #[sea_orm(column_name = "name", default = "")]
    pub name: String,

    // 对应 TypeORM 的 index: number @Column(name: "index_num")
    // 使用 Option<i32> 以匹配 TypeORM 的 number 类型，并映射到 index_num
    #[sea_orm(column_name = "index_num")]
    pub index_num: i32,

    // 对应 TypeORM 的 updateAt: string @Column(name: "update_at")
    #[sea_orm(column_name = "update_at", default = "")]
    pub update_at: String,

    // 对应 TypeORM 的 objects: string @Column(name: "objects", type: "text", default: "")
    #[sea_orm(default = "")]
    pub objects: String,

    // 对应 TypeORM 的 sysvars: string @Column(name: "sysvars", type: "text", default: "")
    #[sea_orm(default = "")]
    pub sysvars: String,

    // 对应 TypeORM 的 modelId: string @Column(name: "model_id", default: "")
    #[sea_orm(column_name = "model_id", default = "")]
    pub model_id: String,

    // 对应 TypeORM 的 userName: string @Column(name: "username", nullable: true)
    // 注意 TypeORM 列名为 "username"，Rust 字段名为 userName
    #[sea_orm(column_name = "username")]
    pub user_name: Option<String>,

    // 对应 TypeORM 的 type: number @Column(name: "type", nullable: true)
    #[sea_orm(column_name = "type")]
    pub type_num: Option<i32>, // Rust 字段名改为 type_num 避免与保留关键字冲突

    // 对应 TypeORM 的 stateIndex: number @Column(name: "state_index", nullable: true)
    #[sea_orm(column_name = "state_index")]
    pub state_index: Option<i32>,

    // 对应 TypeORM 的 stateDesc: string @Column(name: "state_desc", nullable: true)
    #[sea_orm(column_name = "state_desc")]
    pub state_desc: Option<String>,

    // 对应 TypeORM 的 status: string @Column(name: "status", default: "")
    #[sea_orm(default = "")]
    pub status: String,
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
