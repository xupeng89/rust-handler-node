use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_all_detail::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{FromQueryResult, QueryFilter, QueryOrder,QuerySelect, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

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
    pub temperature_equation_proprety: String,
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
    js_name = "ModelComponentChannelDTO"
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
            temperature_equation_proprety: m.temperature_equation_proprety,
        }
    }
}

// ======================================
// Core Service
// ======================================

pub async fn get_one_all_compound_detail_by_id(
    id: String,
) -> Result<Option<ModelComponentAllDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelComponentAllDetailDTO::from))
}

pub async fn get_all_model_compound_channel_count(channel_id: String) -> Result<u64, DbErr> {
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
        .order_by_asc(ModelColumn::SortNum)
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentAllDetailDTO::from)
        .collect())
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
        .order_by_asc(ModelColumn::SortNum)
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
        .order_by_asc(ModelColumn::SortNum)
        .into_model::<ComponentCasNoNameDTO>()
        .all(db)
        .await
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
            temperature_equation_proprety: Set(d.temperature_equation_proprety),
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

pub async fn update_physical_data(id: String, base: String, temp: String) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::update_many()
        .col_expr(ModelColumn::BasePhysicalProperty, Expr::value(base))
        .col_expr(ModelColumn::TemperatureEquationProprety, Expr::value(temp))
        .filter(ModelColumn::Id.eq(id))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}
