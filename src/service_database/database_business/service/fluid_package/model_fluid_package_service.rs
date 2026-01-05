use crate::service_database::database_business::db_business_connection::get_business_db;
use sea_orm::{entity::prelude::*, Set, QueryFilter, ActiveModelTrait, TransactionTrait,FromQueryResult};
use crate::service_database::database_business::entity::fluid_package::model_fluid_package_entity::{
    Entity as PackageEntity, Column as PackageColumn, Model as PackageModel, ActiveModel as PackageActiveModel
};
use crate::service_database::database_business::entity::fluid_package::model_physical_property_calc_entity::{
    Entity as CalcEntity, Column as CalcColumn, Model as CalcModel
};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

// ======================================
// DTOs
// ======================================

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelFluidPackage",
    js_name = "ModelFluidPackageDTO"
)]
pub struct ModelFluidPackageDTO {
    pub id: String,
    pub name: String,
    pub model_id: String,
    pub compound_channel_id: String,
    pub compound_henry_id: String,
    pub property_method_id: i32,
    pub is_default: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelFluidPackage",
    js_name = "ModelFluidPackageUpdateDTO"
)]
pub struct ModelFluidPackageUpdateDTO {
    pub id: String,
    pub name: Option<String>,
    pub property_method_id: Option<i32>,
    pub is_default: Option<i32>,
    pub compound_channel_id: Option<String>,
    pub compound_henry_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelFluidPackage",
    js_name = "ModelPhysicalPropertyCalcDTO"
)]
pub struct ModelPhysicalPropertyCalcDTO {
    pub id: String,
    pub physical_property_name: String,
    pub physical_property_id: String,
    pub fluid_package_id: String,
    pub physical_property_code: String,
    pub calc_code: String,
    pub default_calc_code: String,
    pub key: String,
    pub phase: String,
    pub mixture: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelFluidPackage")]
pub struct PpMethodFunctionDTO {
    pub id: String,
    pub func_code: String,
}

// ======================================
// 1. 流体包 DTO 转换
// ======================================
impl From<PackageModel> for ModelFluidPackageDTO {
    fn from(m: PackageModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            model_id: m.model_id,
            compound_channel_id: m.compound_channel_id,
            compound_henry_id: m.compound_henry_id,
            property_method_id: m.property_method_id,
            is_default: m.is_default,
        }
    }
}

// ======================================
// 2. 物性计算详情 DTO 转换
// ======================================
impl From<CalcModel> for ModelPhysicalPropertyCalcDTO {
    fn from(m: CalcModel) -> Self {
        Self {
            id: m.id,
            physical_property_name: m.physical_property_name,
            physical_property_id: m.physical_property_id,
            fluid_package_id: m.fluid_package_id,
            physical_property_code: m.physical_property_code,
            calc_code: m.calc_code,
            default_calc_code: m.default_calc_code,
            key: m.key,
            phase: m.phase,
            mixture: m.mixture,
        }
    }
}
impl ModelFluidPackageDTO {
    fn into_active_model(self) -> PackageActiveModel {
        PackageActiveModel {
            id: Set(self.id),
            name: Set(self.name),
            model_id: Set(self.model_id),
            compound_channel_id: Set(self.compound_channel_id),
            compound_henry_id: Set(self.compound_henry_id),
            property_method_id: Set(self.property_method_id),
            is_default: Set(self.is_default),
        }
    }
}
// ======================================
// Service 核心逻辑
// ======================================

/// 获取流体包下的所有计算方法
pub async fn get_calc_functions_by_package_id(
    package_id: String,
) -> Result<Vec<ModelPhysicalPropertyCalcDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = CalcEntity::find()
        .filter(CalcColumn::FluidPackageId.eq(package_id))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelPhysicalPropertyCalcDTO::from)
        .collect())
}

pub async fn get_calc_functions_by_package_ids(
    package_ids: Vec<String>,
) -> Result<Vec<ModelPhysicalPropertyCalcDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = CalcEntity::find()
        .filter(CalcColumn::FluidPackageId.is_in(package_ids))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelPhysicalPropertyCalcDTO::from)
        .collect())
}

/// 批量更新计算方法 (对应 TS 的 updateFluidPackageFunctionSelected)
pub async fn update_calc_functions_selected(
    package_id: String,
    list: Vec<PpMethodFunctionDTO>,
) -> Result<(), DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;
    for item in list {
        CalcEntity::update_many()
            .col_expr(CalcColumn::CalcCode, Expr::value(item.func_code))
            .filter(CalcColumn::Id.eq(item.id))
            .filter(CalcColumn::FluidPackageId.eq(package_id.clone()))
            .exec(&txn)
            .await?;
    }
    txn.commit().await?;
    Ok(())
}

/// 维护 isDefault 状态 (将其他设为 0)
pub async fn set_fluid_package_default(model_id: String, target_id: String) -> Result<(), DbErr> {
    let db = get_business_db().await?;
    PackageEntity::update_many()
        .col_expr(PackageColumn::IsDefault, Expr::value(0))
        .filter(PackageColumn::ModelId.eq(model_id))
        .filter(PackageColumn::Id.ne(target_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 局部更新流体包
pub async fn update_fluid_package(data: ModelFluidPackageUpdateDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let existing = PackageEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Fluid package not found".into()))?;

    let mut active: PackageActiveModel = existing.into();
    if let Some(val) = data.name {
        active.name = Set(val);
    }
    if let Some(val) = data.property_method_id {
        active.property_method_id = Set(val);
    }
    if let Some(val) = data.is_default {
        active.is_default = Set(val);
    }
    if let Some(val) = data.compound_channel_id {
        active.compound_channel_id = Set(val);
    }
    if let Some(val) = data.compound_henry_id {
        active.compound_henry_id = Set(val);
    }

    active.update(db).await?;
    Ok(data.id)
}

/// 插入并处理重名 (基础 DB 操作)
pub async fn insert_fluid_package(data: ModelFluidPackageDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    // 注意：Rust 端的重名逻辑建议调用外面传入的已处理好的 Name
    let active: PackageActiveModel = data.into_active_model();
    let res = PackageEntity::insert(active).exec(db).await?;
    Ok(res.last_insert_id)
}

// 模型相关的查询辅助函数
pub async fn get_fluid_package_count(model_id: String, only_default: bool) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let mut query = PackageEntity::find().filter(PackageColumn::ModelId.eq(model_id));
    if only_default {
        query = query.filter(PackageColumn::IsDefault.eq(1));
    }
    query.count(db).await
}
