use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_henry::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{ QueryFilter,FromQueryResult, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelComponentHenry",
    js_name = "ModelComponentHenryDTO"
)]
pub struct ModelComponentHenryDTO {
    pub id: String,
    pub name: String,
    pub compound_channel_id: String,
    pub compound_detail_ids: String,
}

impl From<ModelModel> for ModelComponentHenryDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            compound_channel_id: m.compound_channel_id,
            compound_detail_ids: m.compound_detail_ids,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelComponentHenry")]
pub struct ComponentHenryConnectDTO {
    pub component_henry_id: String,
    pub compound_detail_ids: Vec<Option<String>>,
}

// ======================================
// Core Service
// ======================================

/// 插入多条数据
pub async fn insert_component_henry_list(list: Vec<ModelComponentHenryDTO>) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(false);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            id: Set(d.id),
            name: Set(d.name),
            compound_channel_id: Set(d.compound_channel_id),
            compound_detail_ids: Set(d.compound_detail_ids),
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

/// 查询名称是否重复
pub async fn select_component_henry_by_name(
    name: String,
    channel_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let count = ModelEntity::find()
        .filter(ModelColumn::Name.eq(name))
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .count(db)
        .await?;
    Ok(count > 0)
}

/// 根据 ID 批量删除
pub async fn delete_component_henry_by_ids(ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

/// 更新名称
pub async fn update_component_henry_name(id: String, name: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    ModelEntity::update_many()
        .col_expr(ModelColumn::Name, Expr::value(name))
        .filter(ModelColumn::Id.eq(id.clone()))
        .exec(db)
        .await?;
    Ok(true)
}

/// 根据通道 ID 获取列表
pub async fn select_by_component_channel_id(
    channel_id: String,
) -> Result<Vec<ModelComponentHenryDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelComponentHenryDTO::from).collect())
}

/// 核心业务：连接组分详情 ID 列表 (String 拼接存储)
pub async fn henry_connect_component_detail(msg: ComponentHenryConnectDTO) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    // 过滤掉 None 值的 ID 并用逗号拼接
    let save_details = msg
        .compound_detail_ids
        .into_iter()
        .flatten() // 移除 Option::None
        .collect::<Vec<String>>()
        .join(",");

    let res = ModelEntity::update_many()
        .col_expr(ModelColumn::CompoundDetailIds, Expr::value(save_details))
        .filter(ModelColumn::Id.eq(msg.component_henry_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

/// 根据通道 IDs 批量删除亨利数据
pub async fn delete_by_component_channel_ids(channel_ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::CompoundChannelId.is_in(channel_ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn get_model_component_henry_by_ids(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentHenryDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelComponentHenryDTO::from).collect())
}
