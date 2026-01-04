use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_pf_model_params::{
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
    namespace = "modelPfModelParams",
    js_name = "ModelPfModelParamsDTO"
)]
pub struct ModelPfModelParamsDTO {
    pub id: String,
    pub model_id: String,
    pub code: String,
    pub name: String,
    pub solver_type: String,
    pub params: String,
    pub default_id: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelPfModelParams",
    js_name = "ModelPfModelParamsUpdateDTO"
)]
pub struct ModelPfModelParamsUpdateDTO {
    pub id: String, // 必须提供 ID
    pub model_id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub solver_type: Option<String>,
    pub params: Option<String>,
    pub default_id: Option<i32>,
}

impl From<ModelModel> for ModelPfModelParamsDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            code: m.code,
            name: m.name,
            solver_type: m.solver_type,
            params: m.params,
            default_id: m.default_id,
        }
    }
}

// ======================================
// Core Service
// ======================================

/// 获取所有数据 (根据 model_id)
pub async fn get_all_pf_model_params_by_model_id(
    model_id: String,
) -> Result<Vec<ModelPfModelParamsDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelPfModelParamsDTO::from).collect())
}

/// 插入数据 (批量)
pub async fn insert_pf_model_params_message(
    list: Vec<ModelPfModelParamsDTO>,
) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(true);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            id: Set(d.id),
            model_id: Set(d.model_id),
            code: Set(d.code),
            name: Set(d.name),
            solver_type: Set(d.solver_type),
            params: Set(d.params),
            default_id: Set(d.default_id),
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

/// 更新数据 (局部更新)
pub async fn update_pf_model_params_message(
    data: ModelPfModelParamsUpdateDTO,
) -> Result<String, DbErr> {
    let db = get_business_db().await?;

    // 1. 获取现有记录
    let existing = ModelEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("Params record {} not found", data.id)))?;

    // 2. 转换为 ActiveModel
    let mut active: ActiveModel = existing.into();

    // 3. 只有当 Option 为 Some 时才 Set 值
    if let Some(val) = data.model_id {
        active.model_id = Set(val);
    }
    if let Some(val) = data.code {
        active.code = Set(val);
    }
    if let Some(val) = data.name {
        active.name = Set(val);
    }
    if let Some(val) = data.solver_type {
        active.solver_type = Set(val);
    }
    if let Some(val) = data.params {
        active.params = Set(val);
    }
    if let Some(val) = data.default_id {
        active.default_id = Set(val);
    }

    // 4. 执行更新
    active.update(db).await?;
    Ok(data.id)
}
