use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_system_variable_entity::{
    ActiveModel as ModelSystemActiveModel, Column as ModelSystemColumn,
    Entity as ModelSystemEntity, Model as ModelSystemModel,
};
use sea_orm::{entity::prelude::*, Set, QueryOrder};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object, namespace = "modelSystem", js_name = "ModelSystemVariableDTO")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelSystemVariableDTO {
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub code: String,
    pub value: f64,
    pub sort_id: i32,
}

// Model -> DTO (查询使用)
impl From<ModelSystemModel> for ModelSystemVariableDTO {
    fn from(m: ModelSystemModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            code: m.code,
            value: m.value,
            sort_id: m.sort_id,
        }
    }
}

// DTO -> ActiveModel (插入使用)
impl From<ModelSystemVariableDTO> for ModelSystemActiveModel {
    fn from(data: ModelSystemVariableDTO) -> Self {
        Self {
            id: Set(data.id),
            model_id: Set(data.model_id),
            name: Set(data.name),
            code: Set(data.code),
            value: Set(data.value),
            sort_id: Set(data.sort_id),
        }
    }
}
/// 获取指定模型的所有系统变量
pub async fn get_system_detail_by_model_id(
    model_id: String,
) -> Result<Vec<ModelSystemVariableDTO>, DbErr> {
    let db = get_business_db().await?;

    let results = ModelSystemEntity::find()
        .filter(ModelSystemColumn::ModelId.eq(model_id))
        .order_by_asc(ModelSystemColumn::SortId)
        .all(db)
        .await?;

    Ok(results
        .into_iter()
        .map(ModelSystemVariableDTO::from)
        .collect())
}

/// 批量更新系统变量
pub async fn update_system_detail_batch(
    systems: Vec<ModelSystemVariableDTO>,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    for item in systems {
        // 查找现有记录
        let existing = ModelSystemEntity::find_by_id(item.id.clone())
            .one(db)
            .await?;

        if let Some(model) = existing {
            let mut active_model: ModelSystemActiveModel = model.into();
            active_model.name = Set(item.name);
            active_model.code = Set(item.code);
            active_model.value = Set(item.value);
            active_model.sort_id = Set(item.sort_id);
            active_model.update(db).await?;
        }
    }

    Ok(true)
}

/// 初始化或更新模型系统变量
pub async fn init_model_system_variable_detail(
    systems: Vec<ModelSystemVariableDTO>,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    // 检查该模型是否已有数据
    let exists = ModelSystemEntity::find()
        .filter(ModelSystemColumn::ModelId.eq(model_id.clone()))
        .one(db)
        .await?;

    if exists.is_some() {
        // 如果存在，执行更新逻辑 (Partial Update)
        for item in systems {
            let existing_item = ModelSystemEntity::find_by_id(item.id.clone())
                .one(db)
                .await?;

            if let Some(m) = existing_item {
                let mut active: ModelSystemActiveModel = m.into();
                active.name = Set(item.name);
                active.code = Set(item.code);
                active.value = Set(item.value);
                active.update(db).await?;
            }
        }
    } else {
        // 如果不存在，执行批量插入
        let active_models: Vec<ModelSystemActiveModel> = systems
            .into_iter()
            .map(|mut item| {
                item.model_id = model_id.clone(); // 确保 model_id 一致
                ModelSystemActiveModel::from(item)
            })
            .collect();

        ModelSystemEntity::insert_many(active_models)
            .exec(db)
            .await?;
    }

    Ok(true)
}
