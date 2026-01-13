use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_util_handle::model_global_script_entity::{
    ActiveModel as ScriptActiveModel, Column as ScriptColumn, Entity as ScriptEntity,
    Model as ScriptModel,
};
use napi_derive::napi;
use sea_orm::{ActiveModelTrait, QueryFilter, QueryOrder, FromQueryResult,Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(object, namespace = "modelScript", js_name = "ModelGlobalScriptDTO")]
pub struct ModelGlobalScriptDTO {
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub unit_id: String,
    pub r#type: i32,
    pub index_num: i32,
    pub content: String,
    pub actived: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelScript",
    js_name = "ModelGlobalScriptUpdateDTO"
)]
pub struct ModelGlobalScriptUpdateDTO {
    pub id: String,
    pub model_id: String,
    pub name: Option<String>,
    pub unit_id: Option<String>,
    pub r#type: Option<i32>,
    pub index_num: Option<i32>,
    pub content: Option<String>,
    pub actived: Option<i32>,
}

// ======================================
// 转换逻辑
// ======================================

impl From<ScriptModel> for ModelGlobalScriptDTO {
    fn from(m: ScriptModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            unit_id: m.unit_id,
            r#type: m.r#type,
            index_num: m.index_num,
            content: m.content,
            actived: m.actived,
        }
    }
}

impl ModelGlobalScriptDTO {
    fn into_active_model(self) -> ScriptActiveModel {
        ScriptActiveModel {
            id: Set(self.id),
            model_id: Set(self.model_id),
            name: Set(self.name),
            unit_id: Set(self.unit_id),
            r#type: Set(self.r#type),
            index_num: Set(self.index_num),
            content: Set(self.content),
            actived: Set(self.actived),
        }
    }
}
/// 根据 ID 获取单个脚本
pub async fn get_by_id(id: String) -> Result<Option<ModelGlobalScriptDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelGlobalScriptDTO::from))
}

/// 根据 type 获取列表 (带排序: index_num ASC)
pub async fn get_all_by_type(
    model_id: String,
    type_num: i32,
) -> Result<Vec<ModelGlobalScriptDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::find()
        .filter(ScriptColumn::ModelId.eq(model_id))
        .filter(ScriptColumn::Type.eq(type_num))
        .order_by_asc(ScriptColumn::IndexNum)
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelGlobalScriptDTO::from).collect())
}

/// 根据 IDs 批量获取 (带排序)
pub async fn get_all_by_ids(ids: Vec<String>) -> Result<Vec<ModelGlobalScriptDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::find()
        .filter(ScriptColumn::Id.is_in(ids))
        .order_by_asc(ScriptColumn::IndexNum)
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelGlobalScriptDTO::from).collect())
}

/// 根据 model_id 批量获取 (带排序)
pub async fn get_all_by_model_id(model_id: String) -> Result<Vec<ModelGlobalScriptDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::find()
        .filter(ScriptColumn::ModelId.eq(model_id))
        .order_by_asc(ScriptColumn::IndexNum)
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelGlobalScriptDTO::from).collect())
}

/// 根据单元 ID 获取脚本
pub async fn get_unit_scripts(
    model_id: String,
    unit_id: String,
) -> Result<Vec<ModelGlobalScriptDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::find()
        .filter(ScriptColumn::ModelId.eq(model_id))
        .filter(ScriptColumn::UnitId.eq(unit_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelGlobalScriptDTO::from).collect())
}

/// 创建
pub async fn insert(data: ModelGlobalScriptDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let am = data.into_active_model();
    ScriptEntity::insert(am).exec(db).await?;
    Ok(true)
}

/// 局部更新
pub async fn update(data: ModelGlobalScriptUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = ScriptEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Script not found".into()))?;

    let mut am: ScriptActiveModel = existing.into();

    if let Some(v) = data.name {
        am.name = Set(v);
    }
    if let Some(v) = data.unit_id {
        am.unit_id = Set(v);
    }
    if let Some(v) = data.r#type {
        am.r#type = Set(v);
    }
    if let Some(v) = data.index_num {
        am.index_num = Set(v);
    }
    if let Some(v) = data.content {
        am.content = Set(v);
    }
    if let Some(v) = data.actived {
        am.actived = Set(v);
    }

    am.update(db).await?;
    Ok(true)
}

/// 批量删除
pub async fn delete_by_ids(ids: Vec<String>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ScriptEntity::delete_many()
        .filter(ScriptColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}
