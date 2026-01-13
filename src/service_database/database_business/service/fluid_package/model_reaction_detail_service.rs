use crate::service_database::database_business::db_business_connection::get_business_db;
use sea_orm::{entity::prelude::*, Set, QueryFilter, ActiveModelTrait, FromQueryResult};
use crate::service_database::database_business::entity::fluid_package::model_reaction_detail_entity::{
    Entity as DetailEntity, Column as DetailColumn, Model as DetailModel, ActiveModel as DetailActiveModel
};
use napi_derive::napi;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelReactionDetail",
    js_name = "ModelReactionDetailDTO"
)]
pub struct ModelReactionDetailDTO {
    pub id: String,
    pub reaction_package_id: String,
    pub model_id: String,
    pub name: String,
    pub reaction_type: String,
    pub level: i32,
    pub equation: String,
    pub balance: String,
    pub reaction_heat: String,
    pub list: String,
    pub base_info: String,
    pub reaction_name: String,
    pub conc_basis: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelReactionDetail",
    js_name = "ModelReactionDetailUpdateDTO"
)]
pub struct ModelReactionDetailUpdateDTO {
    pub id: String,
    pub reaction_package_id: Option<String>,
    pub model_id: String,
    pub name: Option<String>,
    pub reaction_type: Option<String>,
    pub level: Option<i32>,
    pub equation: Option<String>,
    pub balance: Option<String>,
    pub reaction_heat: Option<String>,
    pub list: Option<String>,
    pub base_info: Option<String>,
    pub reaction_name: Option<String>,
    pub conc_basis: Option<String>,
}

// ======================================
// 转换逻辑 (Impls)
// ======================================

impl From<DetailModel> for ModelReactionDetailDTO {
    fn from(m: DetailModel) -> Self {
        Self {
            id: m.id,
            reaction_package_id: m.reaction_package_id,
            model_id: m.model_id,
            name: m.name,
            reaction_type: m.reaction_type,
            level: m.level,
            equation: m.equation,
            balance: m.balance,
            reaction_heat: m.reaction_heat,
            list: m.list,
            base_info: m.base_info,
            reaction_name: m.reaction_name,
            conc_basis: m.conc_basis,
        }
    }
}

impl ModelReactionDetailDTO {
    fn into_active_model(self) -> DetailActiveModel {
        DetailActiveModel {
            id: Set(self.id),
            reaction_package_id: Set(self.reaction_package_id),
            model_id: Set(self.model_id),
            name: Set(self.name),
            reaction_type: Set(self.reaction_type),
            level: Set(self.level),
            equation: Set(self.equation),
            balance: Set(self.balance),
            reaction_heat: Set(self.reaction_heat),
            list: Set(self.list),
            base_info: Set(self.base_info),
            reaction_name: Set(self.reaction_name),
            conc_basis: Set(self.conc_basis),
        }
    }
}

// ======================================
// ModelReactionDetail Service
// ======================================

pub async fn insert_model_reaction_detail(
    datas: Vec<ModelReactionDetailDTO>,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let active_list: Vec<DetailActiveModel> =
        datas.into_iter().map(|d| d.into_active_model()).collect();
    DetailEntity::insert_many(active_list).exec(db).await?;
    Ok(true)
}

pub async fn get_model_reaction_detail_by_id(
    id: String,
) -> Result<Option<ModelReactionDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = DetailEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelReactionDetailDTO::from))
}

pub async fn get_model_reaction_details_by_reaction_id(
    reaction_id: String,
) -> Result<Vec<ModelReactionDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = DetailEntity::find()
        .filter(DetailColumn::ReactionPackageId.eq(reaction_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionDetailDTO::from).collect())
}

pub async fn get_model_reaction_details_list_by_reaction_ids(
    reaction_ids: Vec<String>,
) -> Result<Vec<ModelReactionDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = DetailEntity::find()
        .filter(DetailColumn::ReactionPackageId.is_in(reaction_ids))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionDetailDTO::from).collect())
}

pub async fn get_model_reaction_details_list_by_model_id(
    model_id: String,
) -> Result<Vec<ModelReactionDetailDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = DetailEntity::find()
        .filter(DetailColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelReactionDetailDTO::from).collect())
}

pub async fn delete_model_reaction_details_by_ids(ids: Vec<String>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    DetailEntity::delete_many()
        .filter(DetailColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn update_model_reaction_detail(
    data: ModelReactionDetailUpdateDTO,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = DetailEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Detail not found".into()))?;

    let mut active: DetailActiveModel = existing.into();
    // 对应 TS 的 _.omit(modelReactionDetail, ["id"])，即只更新除了 id 以外的字段
    //
    if let Some(val) = data.reaction_package_id {
        active.reaction_package_id = Set(val)
    }
    if let Some(val) = data.name {
        active.name = Set(val)
    }
    if let Some(val) = data.reaction_type {
        active.reaction_type = Set(val)
    }
    if let Some(val) = data.level {
        active.level = Set(val)
    }

    if let Some(val) = data.equation {
        active.equation = Set(val)
    }
    if let Some(val) = data.balance {
        active.balance = Set(val)
    }
    if let Some(val) = data.reaction_heat {
        active.reaction_heat = Set(val)
    }

    if let Some(val) = data.list {
        active.list = Set(val)
    }
    if let Some(val) = data.base_info {
        active.base_info = Set(val)
    }
    if let Some(val) = data.reaction_name {
        active.reaction_name = Set(val)
    }
    if let Some(val) = data.conc_basis {
        active.conc_basis = Set(val)
    }
    active.model_id = Set(data.model_id);

    active.update(db).await?;
    Ok(true)
}

/// 对应 TS 的 checkModelReactionPackage: 统计特定包名和 modelId 下的数量
pub async fn check_model_reaction_package_count(
    package_name: String,
    model_id: String,
) -> Result<u32, DbErr> {
    let db = get_business_db().await?;
    let count = DetailEntity::find()
        .filter(DetailColumn::ReactionName.eq(package_name))
        .filter(DetailColumn::ModelId.eq(model_id))
        .count(db)
        .await?;
    Ok(count as u32)
}
