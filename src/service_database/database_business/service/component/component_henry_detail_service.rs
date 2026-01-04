use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_henry_detail::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{entity::prelude::*, Set, QueryFilter, Condition, FromQueryResult};
use serde::{Deserialize, Serialize};

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelComponentHenryDetail",
    js_name = "ModelComponentHenryDetailDTO"
)]
pub struct ModelComponentHenryDetailDTO {
    pub id: String,
    pub is_default_id: String,
    pub component_i_id: i32,
    pub component_i: String,
    pub component_j_id: i32,
    pub component_j: String,
    pub source_name: String,
    pub aij: f64,
    pub bij: f64,
    pub cij: f64,
    pub dij: f64,
    pub eij: f64,
    pub tlower: f64,
    pub tupper: f64,
    pub is_default: i32,
    pub compound_henry_id: String,
}

impl From<ModelModel> for ModelComponentHenryDetailDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            is_default_id: m.is_default_id,
            component_i_id: m.component_i_id,
            component_i: m.component_i,
            component_j_id: m.component_j_id,
            component_j: m.component_j,
            source_name: m.source_name,
            aij: m.aij,
            bij: m.bij,
            cij: m.cij,
            dij: m.dij,
            eij: m.eij,
            tlower: m.tlower,
            tupper: m.tupper,
            is_default: m.is_default,
            compound_henry_id: m.compound_henry_id,
        }
    }
}

// ======================================
// Core Service
// ======================================

/// 批量插入
pub async fn insert_list(list: Vec<ModelComponentHenryDetailDTO>) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(true);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            id: Set(d.id),
            is_default_id: Set(d.is_default_id),
            component_i_id: Set(d.component_i_id),
            component_i: Set(d.component_i),
            component_j_id: Set(d.component_j_id),
            component_j: Set(d.component_j),
            source_name: Set(d.source_name),
            aij: Set(d.aij),
            bij: Set(d.bij),
            cij: Set(d.cij),
            dij: Set(d.dij),
            eij: Set(d.eij),
            tlower: Set(d.tlower),
            tupper: Set(d.tupper),
            is_default: Set(d.is_default),
            compound_henry_id: Set(d.compound_henry_id),
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

/// 按照 CASNO 删除 (ComponentI 或 ComponentJ 匹配)
pub async fn delete_by_casno(casnos: Vec<String>, henry_id: String) -> Result<u32, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::CompoundHenryId.eq(henry_id))
        .filter(
            Condition::any()
                .add(ModelColumn::ComponentI.is_in(casnos.clone()))
                .add(ModelColumn::ComponentJ.is_in(casnos)),
        )
        .exec(db)
        .await?;
    Ok(res.rows_affected as u32)
}

pub async fn delete_by_compound_henry_ids(henry_ids: Vec<String>) -> Result<u32, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::CompoundHenryId.is_in(henry_ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected as u32)
}

/// 删除单个或多个 ID，并级联删除对应的 is_default_id 记录
pub async fn delete_by_ids_cascade(ids: Vec<String>) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    // 1. 查找这些记录关联的 isDefaultId
    let records = ModelEntity::find()
        .filter(ModelColumn::Id.is_in(ids.clone()))
        .all(db)
        .await?;

    let mut all_ids_to_delete = ids;
    for r in records {
        if !r.is_default_id.is_empty() {
            all_ids_to_delete.push(r.is_default_id);
        }
    }

    // 2. 执行批量删除
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::Id.is_in(all_ids_to_delete))
        .exec(db)
        .await?;
    Ok(res.rows_affected as u32)
}

/// 根据亨利 ID 获取列表 (默认 is_default = 0)
pub async fn select_by_henry_id(
    henry_id: String,
    is_default: i32,
) -> Result<Vec<ModelComponentHenryDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundHenryId.eq(henry_id))
        .filter(ModelColumn::IsDefault.eq(is_default))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentHenryDetailDTO::from)
        .collect())
}

pub async fn select_by_henry_ids(
    henry_ids: Vec<String>,
    is_default: i32,
) -> Result<Vec<ModelComponentHenryDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundHenryId.is_in(henry_ids))
        .filter(ModelColumn::IsDefault.eq(is_default))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentHenryDetailDTO::from)
        .collect())
}

pub async fn select_by_only_henry_ids(
    henry_ids: Vec<String>,
) -> Result<Vec<ModelComponentHenryDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundHenryId.is_in(henry_ids))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentHenryDetailDTO::from)
        .collect())
}

pub async fn select_by_ids(ids: Vec<String>) -> Result<Vec<ModelComponentHenryDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentHenryDetailDTO::from)
        .collect())
}

/// 更新系数信息
pub async fn update_detail(data: ModelComponentHenryDetailDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let existing = ModelEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Detail not found".into()))?;

    let mut active: ActiveModel = existing.into();
    active.aij = Set(data.aij);
    active.bij = Set(data.bij);
    active.cij = Set(data.cij);
    active.dij = Set(data.dij);
    active.eij = Set(data.eij);
    active.tlower = Set(data.tlower);
    active.tupper = Set(data.tupper);
    active.source_name = Set(data.source_name);

    active.update(db).await?;
    Ok(data.id)
}
