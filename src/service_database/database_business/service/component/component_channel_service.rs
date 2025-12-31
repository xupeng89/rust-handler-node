use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::component_channel::model_component_channel::{
    ActiveModel, Column as ModelColumn, Entity as ModelEntity, Model as ModelModel,
};
use napi_derive::napi;
use sea_orm::{ QueryFilter, QueryOrder,FromQueryResult, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

// ======================================
// DTO 定义
// ======================================
#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelComponentChannel",
    js_name = "ModelComponentChannelDTO"
)]
pub struct ModelComponentChannelDTO {
    pub id: String,
    pub ref_name: String,
    pub name: String,
    pub model_id: String,
}

impl From<ModelModel> for ModelComponentChannelDTO {
    fn from(m: ModelModel) -> Self {
        ModelComponentChannelDTO {
            id: m.id,
            ref_name: m.ref_name,
            name: m.name,
            model_id: m.model_id,
        }
    }
}

impl ModelComponentChannelDTO {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id),
            ref_name: Set(self.ref_name),
            name: Set(self.name),
            model_id: Set(self.model_id),
        }
    }
}

// ======================================
// 核心业务逻辑
// ======================================

/// 根据 id 和 modelId 读取一条记录
pub async fn get_first_component_channel(
    id: String,
    model_id: String,
) -> Result<Option<ModelComponentChannelDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::Id.eq(id))
        .filter(ModelColumn::ModelId.eq(model_id))
        .one(db)
        .await?;
    Ok(res.map(ModelComponentChannelDTO::from))
}

/// 根据模版 id 获取所有数据
pub async fn get_all_component_channel_by_model_id(
    model_id: String,
) -> Result<Vec<ModelComponentChannelDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentChannelDTO::from)
        .collect())
}

/// 根据 IDs 集合获取数据 (IN 查询)
pub async fn get_all_component_channel_by_ids(
    ids: Vec<String>,
) -> Result<Vec<ModelComponentChannelDTO>, DbErr> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentChannelDTO::from)
        .collect())
}

/// 随机取一个（对应 TS 的 ASC 排序取第一个）
pub async fn get_component_channel_by_model_first(
    model_id: String,
) -> Result<ModelComponentChannelDTO, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(model_id))
        .order_by_asc(ModelColumn::Id)
        .one(db)
        .await?;
    Ok(res.map(ModelComponentChannelDTO::from).unwrap())
}

/// 插入数据 (逻辑需在 API 层调用名称校验)
pub async fn insert_component_channel(data: ModelComponentChannelDTO) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let active_model = data.into_active_model();
    let res = active_model.insert(db).await?;
    Ok(res.id)
}

/// 批量插入
pub async fn insert_component_channels_copy(
    datas: Vec<ModelComponentChannelDTO>,
) -> Result<bool, DbErr> {
    if datas.is_empty() {
        return Ok(false);
    }
    let db = get_business_db().await?;
    let active_models: Vec<ActiveModel> =
        datas.into_iter().map(|d| d.into_active_model()).collect();
    ModelEntity::insert_many(active_models).exec(db).await?;
    Ok(true) // 根据主键类型调整
}

/// 检查名称是否存在
pub async fn select_component_channel_by_name(
    name: String,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let count = ModelEntity::find()
        .filter(ModelColumn::Name.eq(name))
        .filter(ModelColumn::ModelId.eq(model_id))
        .count(db)
        .await?;
    Ok(count > 0)
}

/// 模糊查询名称 (用于重命名逻辑)
pub async fn get_component_channel_by_name_like(
    name: String,
    model_id: String,
) -> Result<Vec<ModelComponentChannelDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::find()
        .filter(ModelColumn::ModelId.eq(model_id))
        .filter(ModelColumn::Name.contains(&name))
        .all(db)
        .await?;
    Ok(res
        .into_iter()
        .map(ModelComponentChannelDTO::from)
        .collect())
}

/// 批量删除
pub async fn delete_component_channel_by_ids(ids: Vec<String>) -> Result<u64, DbErr> {
    let db = get_business_db().await?;
    let res = ModelEntity::delete_many()
        .filter(ModelColumn::Id.is_in(ids))
        .exec(db)
        .await?;
    Ok(res.rows_affected)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelComponentChannel",
    js_name = "ModelComponentChannelUpdateDTO"
)]
pub struct ModelComponentChannelUpdateDTO {
    pub id: String,
    pub ref_name: Option<String>,
    pub name: Option<String>,
}

/// 更新数据
pub async fn update_component_channel(
    data: ModelComponentChannelUpdateDTO,
    model_id: String,
) -> Result<String, DbErr> {
    let db = get_business_db().await?;
    let existing = ModelEntity::find()
        .filter(ModelColumn::Id.eq(data.id.clone()))
        .filter(ModelColumn::ModelId.eq(model_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Record not found".to_owned()))?;

    let mut active_model: ActiveModel = existing.into();
    if let Some(val) = data.ref_name {
        active_model.ref_name = Set(val);
    }
    if let Some(val) = data.name {
        active_model.name = Set(val);
    }

    let res = active_model.update(db).await?;
    Ok(res.id)
}
