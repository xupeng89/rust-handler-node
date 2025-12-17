use crate::service_database::database_shutter::db_shutter_connection::get_shutter_db;
use napi_derive::napi;

use sea_orm::{
    prelude::Expr, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult,
    IntoActiveModel, QueryFilter, QuerySelect, Set,
};
use serde::Serialize;
// ======================================
use crate::service_database::database_shutter::entity::model_shutter_entity::{
    ActiveModel as ModelShutterActiveModel,
    Column as ModelShutterColumn, // 引入 Column 枚举用于查询
    Entity as ModelShutterEntity,
    Model as ModelShutterModel,
};

// ======================================
// DTO 定义
// ======================================
#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[napi(object, namespace = "shutterHandle")]
pub struct FullShutterModel {
    pub id: String,
    pub name: String,
    pub index_num: i32,
    pub update_at: String,
    pub objects: String,
    pub sysvars: String,
    pub model_id: String,
    pub user_name: Option<String>,
    pub type_num: Option<i32>, // Rust 字段名改为 type_num 避免与保留关键字冲突
    pub state_index: Option<i32>,
    pub state_desc: Option<String>,
    pub base_state_code: String,
}
#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[napi(object, namespace = "shutterHandle")]
pub struct ShutterListItem {
    pub id: String,
    pub name: String,
    pub update_at: String,

    pub index_num: i32,
    pub base_state_code: String,
}

// 读取完整数据 (列表 DTO)
impl From<ModelShutterModel> for ShutterListItem {
    fn from(ele: ModelShutterModel) -> Self {
        ShutterListItem {
            id: ele.id,
            name: ele.name,
            update_at: ele.update_at,
            index_num: ele.index_num,
            base_state_code: ele.base_state_code,
        }
    }
}

// FullCacheData DTO (用于数据传输)
#[derive(Clone, Debug)]
#[napi(object, namespace = "shutterHandle")]
pub struct FullShutterData {
    pub id: String,
    pub name: String,
    pub index_num: i32,
    pub model_id: String,
    pub objects: String,
    pub sysvars: String,
    pub update_at: String,
    pub base_state_code: String,
    pub user_name: Option<String>,
    pub state_index: Option<i32>,
    pub state_desc: Option<String>,
    pub type_num: Option<i32>,
}

// 读取完整数据 (Full DTO)
impl From<ModelShutterModel> for FullShutterData {
    fn from(ele: ModelShutterModel) -> Self {
        FullShutterData {
            id: ele.id,
            name: ele.name,
            index_num: ele.index_num, // 实体中字段应是 index
            model_id: ele.model_id,
            objects: ele.objects,
            sysvars: ele.sysvars,
            update_at: ele.update_at,
            base_state_code: ele.base_state_code,
            user_name: ele.user_name,
            state_index: ele.state_index,
            state_desc: ele.state_desc,
            type_num: ele.type_num,
        }
    }
}

// ======================================
// 数据库操作 (CRUD)
// ======================================

/// 获取快照信息列表 (getAllModelShutterEntityList)
pub async fn get_all_model_shutter_entity_list(
    model_id: String,
) -> Result<Vec<ShutterListItem>, DbErr> {
    let db = get_shutter_db().await?;

    // 使用 .into_model::<ShutterListItem>() 简化转换
    let results = ModelShutterEntity::find()
        .select_only()
        .columns([
            ModelShutterColumn::Id,
            ModelShutterColumn::Name,
            ModelShutterColumn::UpdateAt,
            ModelShutterColumn::IndexNum, // 使用实体中的 index 字段
            ModelShutterColumn::BaseStateCode,
        ])
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .into_model::<ShutterListItem>()
        .all(db)
        .await?;

    Ok(results)
}

/// 获取快照完整信息列表 (getAllModelShutterEntityList)
pub async fn get_all_model_shutter_entity_detail_list(
    model_id: String,
) -> Result<Vec<FullShutterModel>, DbErr> {
    let db = get_shutter_db().await?;

    // 使用 .into_model::<ShutterListItem>() 简化转换
    let results = ModelShutterEntity::find()
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .into_model::<FullShutterModel>()
        .all(db)
        .await?;
    Ok(results)
}

/// 修改快照信息 (updateModelShutterEntity)
/// **修复逻辑:** 检查 DTO 字段是否与当前数据库值**不同**，如果不同，则设置 Set()。
pub async fn update_model_shutter_entity(
    data: FullShutterData, // 使用 DTO 传递要更新的数据
    model_id: String,
) -> Result<i32, DbErr> {
    // 返回更新记录的 ID
    let db = get_shutter_db().await?;

    let current_model = ModelShutterEntity::find_by_id(data.id.clone())
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Model Shutter not found".to_string()))?;
    let result_index_num = current_model.index_num;
    let mut active_model: ModelShutterActiveModel = current_model.into_active_model();

    // 核心修复点：使用 !data.field.eq(active_model.field.as_ref()) 检查差异
    // 并且注意 ActiveModel 中的字段是 ActiveValue，不能直接使用 `==` 或 `eq()` 比较。

    // 1. Objects 字段
    if data.objects != *active_model.objects.as_ref() {
        active_model.objects = Set(data.objects);
    }
    // 2. Sysvars 字段
    if data.sysvars != *active_model.sysvars.as_ref() {
        active_model.sysvars = Set(data.sysvars);
    }
    // 3. Status 字段
    if data.base_state_code != *active_model.base_state_code.as_ref() {
        active_model.base_state_code = Set(data.base_state_code);
    }
    // 4. UpdateAt 字段 (假设您总希望更新时间)
    if data.update_at != *active_model.update_at.as_ref() {
        active_model.update_at = Set(data.update_at);
    }
    // 4. 更新名称
    if data.name != *active_model.name.as_ref() {
        active_model.name = Set(data.name);
    }
    // 您可以根据需要添加其他字段，如 name, user_name 等

    // 4. 执行更新
    active_model.update(db).await?;

    Ok(result_index_num)
}

/// 创建快照信息 (insertModelShutterEntity)
/// 检查 modelId 和 index 是否已存在，若存在则更新 objects 和 sysvars，否则插入新记录
pub async fn insert_model_shutter_entity(
    data: FullShutterData, // 使用 mut 允许修改 model_id
    model_id: String,
) -> Result<i32, DbErr> {
    let db = get_shutter_db().await?;

    let existing_record = ModelShutterEntity::find()
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .filter(ModelShutterColumn::IndexNum.eq(data.index_num)) // 修正为 Index
        .one(db)
        .await?;

    if let Some(record) = existing_record {
        // 2. 如果存在，执行更新逻辑 (update)
        ModelShutterEntity::update_many()
            .col_expr(
                ModelShutterColumn::Objects,
                Expr::value(data.objects), // 将 String 值包裹在 Expr::value 中
            )
            .col_expr(
                ModelShutterColumn::Sysvars,
                Expr::value(data.sysvars), // 将 String 值包裹在 Expr::value 中
            )
            .col_expr(
                ModelShutterColumn::BaseStateCode,
                Expr::value(data.base_state_code), // 将 String 值包裹在 Expr::value 中
            )
            .filter(ModelShutterColumn::Id.eq(record.id)) // 使用 ID 更新更安全
            .exec(db)
            .await?;
    } else {
        // 3. 如果不存在，执行插入逻辑 (insert)
        let active_model = ModelShutterActiveModel {
            id: Set(data.id),
            model_id: Set(data.model_id),
            objects: Set(data.objects),
            sysvars: Set(data.sysvars),
            update_at: Set(data.update_at),
            name: Set(data.name),
            index_num: Set(data.index_num), // 修正为 index
            base_state_code: Set(data.base_state_code),
            ..Default::default()
        };
        active_model.insert(db).await?;
    }

    Ok(data.index_num)
}

/// 仅创建快照信息 (insertModelShutterEntityOnly)
pub async fn insert_model_shutter_entity_only(data: FullShutterData) -> Result<(), DbErr> {
    let db = get_shutter_db().await?;

    let active_model = ModelShutterActiveModel {
        id: Set(data.id),
        model_id: Set(data.model_id),
        objects: Set(data.objects),
        sysvars: Set(data.sysvars),
        update_at: Set(data.update_at),
        // 修正：直接 Set(Option<T>) 类型
        name: Set(data.name),
        index_num: Set(data.index_num),
        base_state_code: Set(data.base_state_code),
        user_name: Set(data.user_name),
        state_index: Set(data.state_index),
        state_desc: Set(data.state_desc),
        type_num: Set(data.type_num),
    };

    active_model.insert(db).await?;
    Ok(())
}

/// 删除快照信息 (deleteModelShutterEntity)
pub async fn delete_model_shutter_entity(id: String, model_id: String) -> Result<u64, DbErr> {
    let db = get_shutter_db().await?;

    let result = ModelShutterEntity::delete_many()
        .filter(ModelShutterColumn::Id.eq(id))
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}

/// 获取一个快照信息 (getModelShutterEntityById)
pub async fn get_model_shutter_entity_by_id(
    id: String,
    model_id: String,
) -> Result<Option<FullShutterData>, DbErr> {
    let db = get_shutter_db().await?;

    let result: Option<ModelShutterModel> = ModelShutterEntity::find_by_id(id)
        .filter(ModelShutterColumn::ModelId.eq(model_id))
        .one(db)
        .await?;

    Ok(result.map(FullShutterData::from))
}

/// 获取一个快照信息 (getModelShutterEntityByIdOnly)
pub async fn get_model_shutter_entity_by_id_only(
    id: String,
) -> Result<Option<FullShutterData>, DbErr> {
    let db = get_shutter_db().await?;

    let result: Option<ModelShutterModel> = ModelShutterEntity::find_by_id(id).one(db).await?;

    Ok(result.map(FullShutterData::from))
}

/// 根据 ID 更新部分信息 (updateModelShutterEntityByIndexOnly)
pub async fn update_model_shutter_entity_by_id_only(
    id: String,
    objects: String,
    sysvars: String,
    base_state_code: String,
) -> Result<u64, DbErr> {
    let db = get_shutter_db().await?;

    let result = ModelShutterEntity::update_many()
        .col_expr(ModelShutterColumn::Objects, Expr::value(objects))
        .col_expr(ModelShutterColumn::Sysvars, Expr::value(sysvars))
        .col_expr(
            ModelShutterColumn::BaseStateCode,
            Expr::value(base_state_code),
        )
        .filter(ModelShutterColumn::Id.eq(id))
        .exec(db)
        .await?;

    Ok(result.rows_affected)
}
