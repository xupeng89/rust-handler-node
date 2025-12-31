use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_all_detail::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{FromQueryResult, Order,QueryFilter, QueryOrder,QuerySelect, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use sea_orm_migration::prelude::NullOrdering;

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelComponentDetail",
    js_name = "ModelComponentAllDetailDTO"
)]
pub struct ModelComponentAllDetailDTO {
    pub id: String,
    pub source_id: String,
    pub source_type: String,
    pub name: String,
    pub formula: String,
    pub cas_no: String,
    pub base_cas_no: String,
    pub number: i32,
    pub internal_name: String,
    pub sort_num: i32,
    pub compound_channel_id: String,
    pub base_physical_property: String,
    pub temperature_equation_property: String,
}

#[derive(FromQueryResult, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelComponentDetail",
    js_name = "ComponentNormalDTO"
)]
pub struct ComponentNormalDTO {
    pub cas_no: String,
    pub name: String,
    pub id: String,
    pub source_id: String,
    pub source_type: String,
}

#[derive(FromQueryResult, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelComponentDetail",
    js_name = "ComponentCasNoNameDTO"
)]
pub struct ComponentCasNoNameDTO {
    pub cas_no: String,
    pub name: String,
    pub internal_name: String,
}

impl From<ModelModel> for ModelComponentAllDetailDTO {
    fn from(m: ModelModel) -> Self {
        Self {
            id: m.id,
            source_id: m.source_id,
            source_type: m.source_type,
            name: m.name,
            formula: m.formula,
            cas_no: m.cas_no,
            base_cas_no: m.base_cas_no,
            number: m.number,
            internal_name: m.internal_name,
            sort_num: m.sort_num,
            compound_channel_id: m.compound_channel_id,
            base_physical_property: m.base_physical_property,
            temperature_equation_property: m.temperature_equation_property,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelCompoundDetail",
    js_name = "ModelCompoundAllDetailUpdateDTO"
)]
pub struct ModelCompoundAllDetailUpdateDTO {
    pub id: String, // ID 是定位记录的必填项
    pub source_id: Option<String>,
    pub source_type: Option<String>,
    pub name: Option<String>,
    pub formula: Option<String>,
    pub cas_no: Option<String>,
    pub base_cas_no: Option<String>,
    pub number: Option<i32>,
    pub internal_name: Option<String>,
    pub sort_num: Option<i32>,
    pub compound_channel_id: Option<String>,
    pub base_physical_property: Option<String>,
    pub temperature_equation_proprety: Option<String>,
}

// ======================================
// Core Service
// ======================================

pub async fn get_one_all_component_detail_by_id(
    id: String,
) -> Result<Option<ModelComponentAllDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelComponentAllDetailDTO::from))
}

pub async fn get_all_model_component_channel_count(channel_id: String) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .count(db)
        .await
}

pub async fn get_all_detail_by_channel_id(
    channel_id: String,
) -> Result<Vec<ModelComponentAllDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentAllDetailDTO::from)
        .collect())
}

pub async fn get_all_component_detail_by_channel_id_vec(
    channel_id: Vec<String>,
) -> Result<Vec<ModelComponentAllDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::CompoundChannelId.is_in(channel_id))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentAllDetailDTO::from)
        .collect())
}

pub async fn select_component_all_detail_have_by_name(
    name: String,
    component_channel_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let count = ModelEntity::find()
        .filter(ModelColumn::Name.eq(name))
        .filter(ModelColumn::CompoundChannelId.eq(component_channel_id))
        .count(db)
        .await?;

    Ok(count > 0)
}

pub async fn get_normal_detail_by_channel_id(
    channel_id: String,
) -> Result<Vec<ComponentNormalDTO>, DbErr> {
    let db = get_business_db().await?;
    ModelEntity::find()
        .select_only()
        .columns([
            ModelColumn::CasNo,
            ModelColumn::Name,
            ModelColumn::Id,
            ModelColumn::SourceId,
            ModelColumn::SourceType,
        ])
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .into_model::<ComponentNormalDTO>()
        .all(db)
        .await
}

pub async fn get_normal_detail_name_casno_by_channel_id(
    channel_id: String,
) -> Result<Vec<ComponentCasNoNameDTO>, DbErr> {
    let db = get_business_db().await?;
    ModelEntity::find()
        .select_only()
        .columns([
            ModelColumn::CasNo,
            ModelColumn::Name,
            ModelColumn::InternalName,
        ])
        .filter(ModelColumn::CompoundChannelId.eq(channel_id))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .into_model::<ComponentCasNoNameDTO>()
        .all(db)
        .await
}

pub async fn find_casno_by_ids(ids: Vec<String>) -> Result<Vec<String>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .select_only()
        .column(ModelColumn::CasNo)
        .filter(ModelColumn::Id.is_in(ids))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .all(db)
        .await?;
    // 从 Model 实体中提取 cas_no 字符串
    Ok(res.into_iter().map(|m| m.cas_no).collect())
}

pub async fn insert_list(list: Vec<ModelComponentAllDetailDTO>) -> Result<bool, DbErr> {
    if list.is_empty() {
        return Ok(false);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> = list
        .into_iter()
        .map(|d| ActiveModel {
            id: Set(d.id),
            source_id: Set(d.source_id),
            source_type: Set(d.source_type),
            name: Set(d.name),
            formula: Set(d.formula),
            cas_no: Set(d.cas_no),
            base_cas_no: Set(d.base_cas_no),
            number: Set(d.number),
            internal_name: Set(d.internal_name),
            sort_num: Set(d.sort_num),
            compound_channel_id: Set(d.compound_channel_id),
            base_physical_property: Set(d.base_physical_property),
            temperature_equation_property: Set(d.temperature_equation_property),
        })
        .collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true)
}

pub async fn delete_by_ids(ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn delete_by_component_channel_ids(channel_ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::CompoundChannelId.is_in(channel_ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn find_by_ids(ids: Vec<String>) -> Result<Vec<ModelComponentAllDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::Id.is_in(ids))
        .order_by_with_nulls(ModelColumn::SortNum, Order::Asc, NullOrdering::Last)
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentAllDetailDTO::from)
        .collect())
}

pub async fn update_physical_data(id: String, base: String, temp: String) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::update_many()
        .col_expr(ModelColumn::BasePhysicalProperty, Expr::value(base))
        .col_expr(ModelColumn::TemperatureEquationProperty, Expr::value(temp))
        .filter(ModelColumn::Id.eq(id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

pub async fn update_compound_all_detail_data(
    data: ModelCompoundAllDetailUpdateDTO,
) -> Result<String, DbErr> {
    let db = get_business_db().await?;

    // 1. 获取现有记录
    let existing = ModelEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Record not found".to_owned()))?;

    // 2. 转换为 ActiveModel
    let mut active_model: ActiveModel = existing.into();

    // 3. 只有当 Option 为 Some 时才设置字段（SeaORM 会自动追踪改变的字段）
    if let Some(val) = data.source_id {
        active_model.source_id = Set(val);
    }
    if let Some(val) = data.source_type {
        active_model.source_type = Set(val);
    }
    if let Some(val) = data.name {
        active_model.name = Set(val);
    }
    if let Some(val) = data.formula {
        active_model.formula = Set(val);
    }
    if let Some(val) = data.cas_no {
        active_model.cas_no = Set(val);
    }
    if let Some(val) = data.base_cas_no {
        active_model.base_cas_no = Set(val);
    }
    if let Some(val) = data.number {
        active_model.number = Set(val);
    }
    if let Some(val) = data.internal_name {
        active_model.internal_name = Set(val);
    }
    if let Some(val) = data.sort_num {
        active_model.sort_num = Set(val);
    }
    if let Some(val) = data.compound_channel_id {
        active_model.compound_channel_id = Set(val);
    }
    if let Some(val) = data.base_physical_property {
        active_model.base_physical_property = Set(val);
    }
    if let Some(val) = data.temperature_equation_proprety {
        active_model.temperature_equation_property = Set(val);
    }

    // 4. 执行更新
    let updated = active_model.update(db).await?;
    Ok(updated.id)
}
