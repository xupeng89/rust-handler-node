use crate::service_database::database_business::db_business_connection::get_business_db;
use sea_orm::{entity::prelude::*, Set, ActiveModelTrait, QueryFilter,FromQueryResult};
use crate::service_database::database_business::entity::model_util_handle::model_initialize_data_in_cold_state_entity::{
    Entity as ColdStateEntity, Column as ColdStateColumn, Model as ColdStateModel, ActiveModel as ColdStateActiveModel
};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelColdState",
    js_name = "ModelInitializeDataInColdStateDTO"
)]
pub struct ModelInitializeDataInColdStateDTO {
    pub id: i32,
    pub name: String,
    pub model_id: String,
    pub material_object: String,
    pub fluid_package_id: String,
    pub graphic_element_model_list: String, // 存储为 JSON String
    pub config_msg: String,                 // 存储为 JSON String
    pub is_default: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelColdState",
    js_name = "ModelColdStateUpdateDTO"
)]
pub struct ModelColdStateUpdateDTO {
    pub id: i32,
    pub name: Option<String>,
    pub model_id: String, // 用于定位
    pub material_object: Option<String>,
    pub fluid_package_id: Option<String>,
    pub graphic_element_model_list: Option<String>,
    pub config_msg: Option<String>,
    pub is_default: Option<i32>,
}

// ======================================
// 转换逻辑
// ======================================

impl From<ColdStateModel> for ModelInitializeDataInColdStateDTO {
    fn from(m: ColdStateModel) -> Self {
        Self {
            id: m.id,
            name: m.name,
            model_id: m.model_id,
            material_object: m.material_object,
            fluid_package_id: m.fluid_package_id,
            graphic_element_model_list: m.graphic_element_model_list,
            config_msg: m.config_msg,
            is_default: m.is_default,
        }
    }
}

/// 根据 ModelID 获取列表
pub async fn get_list_by_model_id(
    model_id: String,
) -> Result<Vec<ModelInitializeDataInColdStateDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ColdStateEntity::find()
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelInitializeDataInColdStateDTO::from)
        .collect())
}

/// 根据 id 和 ModelID 获取列表
pub async fn get_one_by_id_model_id(
    id: i32,
    model_id: String,
) -> Result<Option<ModelInitializeDataInColdStateDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ColdStateEntity::find_by_id(id)
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .one(db)
        .await?;
    Ok(res.map(ModelInitializeDataInColdStateDTO::from))
}

/// 获取默认配置
pub async fn get_default_by_model_id(
    model_id: String,
) -> Result<Option<ModelInitializeDataInColdStateDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ColdStateEntity::find()
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .filter(ColdStateColumn::IsDefault.eq(1))
        .one(db)
        .await?;
    Ok(res.map(ModelInitializeDataInColdStateDTO::from))
}

/// 创建初始化数据 (不带 ID，主键自增)
pub async fn create(data: ModelInitializeDataInColdStateDTO) -> Result<i32, DbErr> {
    let db = get_business_db().await?;
    let am = ColdStateActiveModel {
        name: Set(data.name),
        model_id: Set(data.model_id),
        material_object: Set(data.material_object),
        fluid_package_id: Set(data.fluid_package_id),
        graphic_element_model_list: Set(data.graphic_element_model_list),
        config_msg: Set(data.config_msg),
        is_default: Set(data.is_default),
        ..Default::default()
    };
    let res = ColdStateEntity::insert(am).exec(db).await?;
    Ok(res.last_insert_id)
}

/// 将该 Model 下的所有数据设为非默认 (isDefault = 0)
pub async fn set_others_not_default(model_id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    ColdStateEntity::update_many()
        .col_expr(ColdStateColumn::IsDefault, Expr::value(0))
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn update_is_default_by_model_id(
    model_id: String,
    is_default: i32,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    ColdStateEntity::update_many()
        .col_expr(ColdStateColumn::IsDefault, Expr::value(is_default))
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;

    Ok(true)
}

/// 局部更新
pub async fn update(data: ModelColdStateUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = ColdStateEntity::find()
        .filter(ColdStateColumn::Id.eq(data.id))
        .filter(ColdStateColumn::ModelId.eq(data.model_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Cold state data not found".into()))?;

    let mut am: ColdStateActiveModel = existing.into();
    if let Some(v) = data.name {
        am.name = Set(v);
    }
    if let Some(v) = data.material_object {
        am.material_object = Set(v);
    }
    if let Some(v) = data.fluid_package_id {
        am.fluid_package_id = Set(v);
    }
    if let Some(v) = data.graphic_element_model_list {
        am.graphic_element_model_list = Set(v);
    }
    if let Some(v) = data.config_msg {
        am.config_msg = Set(v);
    }
    if let Some(v) = data.is_default {
        am.is_default = Set(v);
    }

    am.update(db).await?;
    Ok(true)
}

/// 删除
pub async fn delete(id: i32, model_id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ColdStateEntity::delete_many()
        .filter(ColdStateColumn::Id.eq(id))
        .filter(ColdStateColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}
