use crate::service_database::database_business::db_business_connection::get_business_db;
use sea_orm::{
    ActiveModelTrait, FromQueryResult, QueryFilter, Set, TransactionTrait, entity::prelude::*,
};
// 假设这是反应包实体的路径
use crate::service_database::database_business::entity::fluid_package::model_reaction_package_entity::{
    Entity as ReactionEntity, Column as ReactionColumn, Model as ReactionModel, ActiveModel as ReactionActiveModel
};
// 假设这是反应包详情实体的路径（对应 TS 中的 getModelReactionDetailQueryBuilder）
use crate::service_database::database_business::entity::fluid_package::model_reaction_detail_entity::{
    Entity as DetailEntity, Column as DetailColumn
};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelReaction",
    js_name = "ModelReactionPackageDTO"
)]
pub struct ModelReactionPackageDTO {
    pub id: String,
    pub reaction_name: String,
    pub model_id: String,
    pub compound_channel_id: String,
    pub fluid_package_ids: String,
    pub reaction_package_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelReaction",
    js_name = "ModelReactionPackageUpdateDTO"
)]
pub struct ModelReactionPackageUpdateDTO {
    pub id: String,
    pub reaction_name: Option<String>,
    pub model_id: Option<String>,
    pub compound_channel_id: Option<String>,
    pub fluid_package_ids: Option<String>,
    pub reaction_package_type: Option<String>,
}

// ======================================
// 转换逻辑 (Impls)
// ======================================

impl From<ReactionModel> for ModelReactionPackageDTO {
    fn from(m: ReactionModel) -> Self {
        Self {
            id: m.id,
            reaction_name: m.reaction_name,
            model_id: m.model_id,
            compound_channel_id: m.compound_channel_id,
            fluid_package_ids: m.fluid_package_ids,
            reaction_package_type: m.reaction_package_type,
        }
    }
}

impl ModelReactionPackageDTO {
    fn into_active_model(self) -> ReactionActiveModel {
        ReactionActiveModel {
            id: Set(self.id),
            reaction_name: Set(self.reaction_name),
            model_id: Set(self.model_id),
            compound_channel_id: Set(self.compound_channel_id),
            fluid_package_ids: Set(self.fluid_package_ids),
            reaction_package_type: Set(self.reaction_package_type),
        }
    }
}

/// 根据 ID 获取反应包
pub async fn get_model_reaction_by_id(
    id: String,
) -> Result<Option<ModelReactionPackageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ReactionEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelReactionPackageDTO::from))
}

/// 根据 ModelId 获取所有反应包
pub async fn get_model_reactions_by_model_id(
    model_id: String,
) -> Result<Vec<ModelReactionPackageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ReactionEntity::find()
        .filter(ReactionColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionPackageDTO::from).collect())
}

/// 根据 ompoundChannelId 获取反应包
pub async fn get_model_reactions_by_channel_id(
    channel_id: String,
) -> Result<Vec<ModelReactionPackageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ReactionEntity::find()
        .filter(ReactionColumn::CompoundChannelId.eq(channel_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionPackageDTO::from).collect())
}

/// 根据 ids ModelId 获取所有反应包
pub async fn get_model_reactions_by_ids_and_model_id(
    ids: Vec<String>,
    model_id: String,
) -> Result<Vec<ModelReactionPackageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ReactionEntity::find()
        .filter(ReactionColumn::Id.is_in(ids))
        .filter(ReactionColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionPackageDTO::from).collect())
}

/// 模糊查询反应名称 (对应 TS 的 getModelReactionByName)
pub async fn get_model_reaction_by_name_like(
    name: String,
    model_id: String,
) -> Result<Vec<ModelReactionPackageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ReactionEntity::find()
        .filter(ReactionColumn::ModelId.eq(model_id))
        .filter(ReactionColumn::ReactionName.like(format!("%{}%", name)))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionPackageDTO::from).collect())
}

/// 批量插入反应包
pub async fn insert_model_reaction_list(
    datas: Vec<ModelReactionPackageDTO>,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let active_list: Vec<ReactionActiveModel> =
        datas.into_iter().map(|d| d.into_active_model()).collect();

    ReactionEntity::insert_many(active_list).exec(db).await?;
    Ok(true)
}

/// 更新反应包
pub async fn update_model_reaction(data: ModelReactionPackageUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = ReactionEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Reaction package not found".into()))?;

    let mut active: ReactionActiveModel = existing.into();
    if let Some(val) = data.reaction_name {
        active.reaction_name = Set(val)
    }
    if let Some(val) = data.compound_channel_id {
        active.compound_channel_id = Set(val)
    }
    if let Some(val) = data.fluid_package_ids {
        active.fluid_package_ids = Set(val)
    }
    if let Some(val) = data.reaction_package_type {
        active.reaction_package_type = Set(val)
    }

    active.update(db).await?;
    Ok(true)
}

/// 批量删除反应包及其关联的详情 (对应 TS 的 deleteModelReactionsByIds)
pub async fn delete_model_reactions_by_ids(ids: Vec<String>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    // 1. 删除详情表关联数据
    DetailEntity::delete_many()
        .filter(DetailColumn::ReactionPackageId.is_in(ids.clone()))
        .exec(&txn)
        .await?;

    // 2. 删除反应包主表数据
    ReactionEntity::delete_many()
        .filter(ReactionColumn::Id.is_in(ids))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok(true)
}
