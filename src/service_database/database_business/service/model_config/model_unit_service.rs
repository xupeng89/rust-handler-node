use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_unit_item_entity::{
    ActiveModel as ItemActiveModel, Column as ItemColumn, Entity as ItemEntity, Model as ItemModel,
};
use crate::service_database::database_business::entity::model_config::model_unit_set_entity::{
    ActiveModel as SetActiveModel, Column as SetColumn, Entity as SetEntity, Model as SetModel,
};
use napi_derive::napi;
use sea_orm::{Set, TransactionTrait, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[napi(object, namespace = "modelUnit", js_name = "ModelUnitSetDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelUnitSetDTO {
    pub id: String,
    pub model_id: String,
    pub en_name: String,
    pub name: String,
    pub code: String,
    pub status: i32,
    pub is_default: i32,
}

// Model -> DTO (查询使用)
impl From<SetModel> for ModelUnitSetDTO {
    fn from(m: SetModel) -> Self {
        ModelUnitSetDTO {
            id: m.id,
            model_id: m.model_id,
            en_name: m.en_name,
            name: m.name,
            code: m.code,
            status: m.status,
            is_default: m.is_default,
        }
    }
}

#[napi(object, namespace = "modelUnit", js_name = "ModelUnitItemDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelUnitItemDTO {
    pub id: String,
    pub set_id: String,
    pub model_id: String,
    pub code: String,
    pub value: String,
}

// Model -> DTO (查询使用)
impl From<ItemModel> for ModelUnitItemDTO {
    fn from(m: ItemModel) -> Self {
        ModelUnitItemDTO {
            id: m.id,
            set_id: m.set_id,
            model_id: m.model_id,
            code: m.code,
            value: m.value,
        }
    }
}

// DTO -> ActiveModel (用于 Set)
impl From<ModelUnitSetDTO> for SetActiveModel {
    fn from(data: ModelUnitSetDTO) -> Self {
        Self {
            id: Set(data.id),
            model_id: Set(data.model_id),
            en_name: Set(data.en_name),
            name: Set(data.name),
            code: Set(data.code),
            status: Set(data.status),
            is_default: Set(data.is_default),
        }
    }
}

// DTO -> ActiveModel (用于 Item)
impl From<ModelUnitItemDTO> for ItemActiveModel {
    fn from(data: ModelUnitItemDTO) -> Self {
        Self {
            id: Set(data.id),
            set_id: Set(data.set_id),
            model_id: Set(data.model_id),
            code: Set(data.code),
            value: Set(data.value),
        }
    }
}

#[napi(object, namespace = "modelUnit", js_name = "UnitFullDataDTO")]
pub struct UnitFullDataDTO {
    pub set: ModelUnitSetDTO,
    pub items: Vec<ModelUnitItemDTO>,
}

#[napi(object, namespace = "modelUnit", js_name = "ModelUnitSetUpdateDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelUnitSetUpdateDTO {
    pub id: String,       // 必传，过滤使用
    pub model_id: String, // 必传，过滤使用
    pub en_name: Option<String>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub status: Option<i32>,
    pub is_default: Option<i32>,
}

#[napi(
    object,
    namespace = "modelUnit",
    js_name = "ModelUnitSetItemsUpdateDTO"
)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelUnitSetItemsUpdateDTO {
    pub id: String,
    pub en_name: String,
    pub name: String,
    pub code: String,
    pub status: i32, // 0 | 1
    pub model_id: String,
    pub is_default: i32,              // 0 | 1
    pub items: Vec<ModelUnitItemDTO>, // 关联的 items 数组
}

pub async fn get_model_unit_set_with_items(id: String) -> Result<UnitFullDataDTO, DbErr> {
    let db = get_business_db().await?;

    // 使用 ok_or 替代 Option，直接向上抛出错误
    let set: SetModel = SetEntity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("ID {} not found", id)))?;

    let items = ItemEntity::find()
        .filter(ItemColumn::SetId.eq(id))
        .all(db)
        .await?;

    Ok(UnitFullDataDTO {
        set: ModelUnitSetDTO::from(set),
        items: items.into_iter().map(ModelUnitItemDTO::from).collect(),
    })
}

/// 对应 getModelUnitSetOneById
pub async fn get_model_unit_set_one_by_id(id: String) -> Result<ModelUnitSetDTO, DbErr> {
    let db = get_business_db().await?;

    let res = SetEntity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "UnitSet with ID {} not found",
            id
        )))?;

    Ok(ModelUnitSetDTO::from(res))
}

pub async fn get_model_unit_set_one_by_model_id_and_default(
    model_id: String,
) -> Result<ModelUnitSetDTO, DbErr> {
    let db = get_business_db().await?;

    let res = SetEntity::find()
        .filter(SetColumn::ModelId.eq(model_id.clone()))
        .filter(SetColumn::IsDefault.eq(1))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound(format!(
            "UnitSet with ID {} not found",
            model_id
        )))?;

    Ok(ModelUnitSetDTO::from(res))
}

pub async fn get_model_unit_set_all_by_model_id(
    model_id: String,
) -> Result<Vec<ModelUnitSetDTO>, DbErr> {
    let db = get_business_db().await?;

    let res = SetEntity::find()
        .filter(SetColumn::ModelId.eq(model_id.clone()))
        .all(db)
        .await?;

    Ok(res.into_iter().map(ModelUnitSetDTO::from).collect())
}

pub async fn get_model_unit_items_by_set_id_and_model_id(
    set_id: String,
    model_id: String,
) -> Result<Vec<ModelUnitItemDTO>, DbErr> {
    let db = get_business_db().await?;

    let res = ItemEntity::find()
        .filter(ItemColumn::SetId.eq(set_id))
        .filter(ItemColumn::ModelId.eq(model_id))
        .all(db)
        .await?;

    Ok(res.into_iter().map(ModelUnitItemDTO::from).collect())
}

/// 更新 UnitSet (含默认值排他性逻辑)
pub async fn update_unit_set_logic(data: ModelUnitSetUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    // 1. 查找现有记录 (使用 id 和 model_id 过滤)
    let existing = SetEntity::find()
        .filter(SetColumn::Id.eq(data.id.clone()))
        .filter(SetColumn::ModelId.eq(data.model_id.clone()))
        .one(&txn)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound(format!("UnitSet {} not found", data.id)))?;

    // 2. 处理默认值排他性逻辑
    // 如果前端明确传了 is_default 且值为 1
    if let Some(1) = data.is_default {
        SetEntity::update_many()
            .col_expr(SetColumn::IsDefault, Expr::value(0))
            .filter(SetColumn::ModelId.eq(data.model_id.clone()))
            .filter(SetColumn::Id.ne(data.id.clone()))
            .exec(&txn)
            .await?;
    }

    // 3. 转换为 ActiveModel 并进行局部更新
    let mut active: SetActiveModel = existing.into();

    if let Some(val) = data.en_name {
        active.en_name = Set(val);
    }
    if let Some(val) = data.name {
        active.name = Set(val);
    }
    if let Some(val) = data.code {
        active.code = Set(val);
    }
    if let Some(val) = data.status {
        active.status = Set(val);
    }
    if let Some(val) = data.is_default {
        active.is_default = Set(val);
    }

    // 执行更新 SQL (SeaORM 只会 SET 那些被标记为 Set(val) 的字段)
    active.update(&txn).await?;

    txn.commit().await?;
    Ok(true)
}

/// 插入新单位集合及关联项 (带事务)
pub async fn insert_unit_model_full(
    set_data: ModelUnitSetDTO,
    items: Vec<ModelUnitItemDTO>,
) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    let set_id = set_data.id.clone();

    let active_set: SetActiveModel = set_data.into();
    SetEntity::insert(active_set).exec(&txn).await?;

    if !items.is_empty() {
        let active_items: Vec<ItemActiveModel> = items.into_iter().map(|i| i.into()).collect();
        ItemEntity::insert_many(active_items).exec(&txn).await?;
    }

    txn.commit().await?;
    Ok(set_id)
}

/// 插入新单位集合及关联项 (带事务)
pub async fn insert_model_unit_set_only(set_data: Vec<ModelUnitSetDTO>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    let active_set: Vec<SetActiveModel> = set_data.into_iter().map(Into::into).collect();
    SetEntity::insert_many(active_set).exec(&txn).await?;

    txn.commit().await?;
    Ok(true)
}

/// 对应 deleteAllModelUnit
pub async fn delete_all_model_unit_by_id(id: String) -> Result<u32, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    ItemEntity::delete_many()
        .filter(ItemColumn::SetId.eq(id.clone()))
        .exec(&txn)
        .await?;

    let delete_res = SetEntity::delete_by_id(id).exec(&txn).await?;

    txn.commit().await?;

    Ok(delete_res.rows_affected as u32)
}
/// 对应 updateUnitCorrelation
/// 更新单位集基本信息及其关联的单位项值
pub async fn update_unit_all_items(
    unit_set: ModelUnitSetItemsUpdateDTO,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let txn = db.begin().await?;

    // 1. 更新主表 ModelUnitSet 的 code (以及其他根据需要更新的字段)
    // 这里采用 update_many 配合 filter，不需要先查询，性能更好
    SetEntity::update_many()
        .col_expr(SetColumn::Code, Expr::value(unit_set.code))
        .filter(SetColumn::Id.eq(unit_set.id.clone()))
        .filter(SetColumn::ModelId.eq(model_id.clone()))
        .exec(&txn)
        .await?;

    // 2. 循环更新关联的 ModelUnitItem
    // 注意：Rust 的 for 循环配合 ? 可以完美处理异步错误并支持事务回滚
    for item in unit_set.items {
        ItemEntity::update_many()
            .col_expr(ItemColumn::Value, Expr::value(item.value))
            .filter(ItemColumn::SetId.eq(unit_set.id.clone()))
            .filter(ItemColumn::Code.eq(item.code))
            .filter(ItemColumn::ModelId.eq(model_id.clone()))
            .exec(&txn)
            .await?;
    }

    // 3. 提交事务
    txn.commit().await?;

    Ok(true)
}
