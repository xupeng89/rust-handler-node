use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_oil::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, FromQueryResult, QueryFilter, Set, TransactionTrait, entity::prelude::*,
};
use serde::{Deserialize, Serialize};

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelComponentOil",
    js_name = "ModelComponentOilDTO"
)]
pub struct ModelComponentOilDTO {
    pub id: String,
    pub compound_channel_id: String,
    pub name: String,
    pub internal_name: String,
    pub formula: String,
    pub cas_no: String,
    pub group: i32,
    pub base_physical_property: String,
    pub default_physical_property: String,
    pub temperature_equation_property: String,
}

impl From<ModelModel> for ModelComponentOilDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            compound_channel_id: m.compound_channel_id,
            name: m.name,
            internal_name: m.internal_name,
            formula: m.formula,
            cas_no: m.cas_no,
            group: m.group,
            base_physical_property: m.base_physical_property,
            default_physical_property: m.default_physical_property,
            temperature_equation_property: m.temperature_equation_property,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelComponentOil",
    js_name = "ModelComponentOilUpdateDTO"
)]
pub struct ModelComponentOilUpdateDTO {
    pub id: String, // ID 是必须的，用于定位记录
    pub compound_channel_id: Option<String>,
    pub name: Option<String>,
    pub internal_name: Option<String>,
    pub formula: Option<String>,
    pub cas_no: Option<String>,
    pub group: Option<i32>,
    pub base_physical_property: Option<String>,
    pub default_physical_property: Option<String>,
    pub temperature_equation_property: Option<String>,
}

// ======================================
// Core Service
// ======================================

/// 批量插入
pub async fn insert_model_component_oil(list: Vec<ModelComponentOilDTO>) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(true);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            id: Set(d.id),
            compound_channel_id: Set(d.compound_channel_id),
            name: Set(d.name),
            internal_name: Set(d.internal_name),
            formula: Set(d.formula),
            cas_no: Set(d.cas_no),
            group: Set(d.group),
            base_physical_property: Set(d.base_physical_property),
            default_physical_property: Set(d.default_physical_property),
            temperature_equation_property: Set(d.temperature_equation_property),
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

pub async fn get_model_component_oil_by_id(
    id: String,
) -> Result<Option<ModelComponentOilDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelComponentOilDTO::from))
}

pub async fn get_oils_by_channel_id(
    channel_id: String,
) -> Result<Vec<ModelComponentOilDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelComponentOilDTO::from).collect())
}

pub async fn get_oils_by_channel_ids(
    channel_ids: Vec<String>,
) -> Result<Vec<ModelComponentOilDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.is_in(channel_ids))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelComponentOilDTO::from).collect())
}

pub async fn get_oils_by_component_cas_nos(
    cas_no: Vec<String>,
) -> Result<Vec<ModelComponentOilDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CasNo.is_in(cas_no))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelComponentOilDTO::from).collect())
}

/// 更新单条数据 (对应 TS 中的 _.omit(item, ["id"]))
pub async fn update_model_component_oil(data: ModelComponentOilUpdateDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;

    // 1. 先查询现有数据
    let existing = ModelEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!("Oil {} not found", data.id)))?;

    // 2. 转换为 ActiveModel
    let mut active: ActiveModel = existing.into();

    // 3. 只有当 Option 为 Some 时才 Set 值
    if let Some(val) = data.compound_channel_id {
        active.compound_channel_id = Set(val);
    }
    if let Some(val) = data.name {
        active.name = Set(val);
    }
    if let Some(val) = data.internal_name {
        active.internal_name = Set(val);
    }
    if let Some(val) = data.formula {
        active.formula = Set(val);
    }
    if let Some(val) = data.cas_no {
        active.cas_no = Set(val);
    }
    if let Some(val) = data.group {
        active.group = Set(val);
    }
    if let Some(val) = data.base_physical_property {
        active.base_physical_property = Set(val);
    }
    if let Some(val) = data.default_physical_property {
        active.default_physical_property = Set(val);
    }
    if let Some(val) = data.temperature_equation_property {
        active.temperature_equation_property = Set(val);
    }

    // 4. 执行更新 (SeaORM 只会更新改变的字段)
    active.update(db).await?;
    Ok(data.id)
}

/// 批量更新 (在事务中处理)
pub async fn update_model_component_oils(
    list: Vec<ModelComponentOilUpdateDTO>,
) -> Result<Vec<String>, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;
    let mut results = Vec::new();

    for item in list {
        let id = item.id.clone();
        let existing = ModelEntity::find_by_id(id.clone())
            .one(&txn)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Oil {} not found", id)))?;

        let mut active: ActiveModel = existing.into();

        if let Some(val) = item.compound_channel_id {
            active.compound_channel_id = Set(val);
        }
        if let Some(val) = item.name {
            active.name = Set(val);
        }
        if let Some(val) = item.internal_name {
            active.internal_name = Set(val);
        }
        if let Some(val) = item.formula {
            active.formula = Set(val);
        }
        if let Some(val) = item.cas_no {
            active.cas_no = Set(val);
        }
        if let Some(val) = item.group {
            active.group = Set(val);
        }
        if let Some(val) = item.base_physical_property {
            active.base_physical_property = Set(val);
        }
        if let Some(val) = item.default_physical_property {
            active.default_physical_property = Set(val);
        }
        if let Some(val) = item.temperature_equation_property {
            active.temperature_equation_property = Set(val);
        }

        active.update(&txn).await?;
        results.push(id);
    }

    txn.commit().await?;
    Ok(results)
}

pub async fn delete_oils_by_ids(ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn delete_oils_by_channel_ids(channel_ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::CompoundChannelId.is_in(channel_ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}
