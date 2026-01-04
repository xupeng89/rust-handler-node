use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_pf_var_default_params_entity::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{ActiveModelTrait, FromQueryResult, QueryFilter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelPfVarParams",
    js_name = "ModelPfVarParamsDTO"
)]
pub struct ModelPfVarParamsDTO {
    pub id: i32,
    pub model_id: String,
    pub name: String,
    pub delta_max: String,
    pub min: String,
    pub max: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelPfVarParams",
    js_name = "ModelPfVarParamsUpdateDTO"
)]
pub struct ModelPfVarParamsUpdateDTO {
    pub model_id: String, // 用于定位的必填项
    pub name: String,     // 用于定位的必填项
    pub delta_max: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
}

impl From<ModelModel> for ModelPfVarParamsDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            delta_max: m.delta_max,
            min: m.min,
            max: m.max,
        }
    }
}

// ======================================
// Core Service
// ======================================

/// 根据 model_id 获取所有记录
pub async fn get_params_by_model_id(model_id: String) -> Result<Vec<ModelPfVarParamsDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelPfVarParamsDTO::from).collect())
}

/// 根据 model_id 删除所有记录
pub async fn delete_params_by_model_id(model_id: String) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

/// 批量插入记录
pub async fn insert_pf_var_params(list: Vec<ModelPfVarParamsDTO>) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(true);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            model_id: Set(d.model_id),
            name: Set(d.name),
            delta_max: Set(d.delta_max),
            min: Set(d.min),
            max: Set(d.max),
            ..Default::default() // id 通常是自增的
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

/// 根据 Name 和 ModelId 更新记录 (局部更新)
pub async fn update_pf_var_params_by_name(data: ModelPfVarParamsUpdateDTO) -> Result<u64, DbErr> {
    let db = get_business_db().await?;

    // 1. 查找符合条件的现有记录
    let existing = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(data.model_id.clone()))
        .filter(ModelColumn::Name.eq(data.name.clone()))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Record not found by name and model_id".into()))?;

    // 2. 转换为 ActiveModel
    let mut active: ActiveModel = existing.into();

    // 3. 只有当传入字段为 Some 时才更新
    if let Some(val) = data.delta_max {
        active.delta_max = Set(val);
    }
    if let Some(val) = data.min {
        active.min = Set(val);
    }
    if let Some(val) = data.max {
        active.max = Set(val);
    }

    // 4. 执行更新
    let updated = active.update(db).await?;
    Ok(updated.id as u64)
}
