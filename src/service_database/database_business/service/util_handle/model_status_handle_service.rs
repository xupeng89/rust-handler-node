use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_util_handle::model_status_information_entity::{
    Column as InforColumn, Entity as InforEntity,
    Model as InforModel,
};
use crate::service_database::database_business::entity::model_util_handle::model_status_params_entity::{
    ActiveModel as ParamsActiveModel, Column as ParamsColumn, Entity as ParamsEntity,
    Model as ParamsModel,
};
use chrono::Utc;
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, QueryFilter, QueryOrder,DatabaseTransaction, Set, entity::prelude::*,TransactionError,
    sea_query::Expr,TransactionTrait
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::tool_handle::json_status_handle::{pack_to_storage_handle, unpack_from_storage_handle};

// --- Information DTO ---
#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelStatus")]
pub struct ModelStatusInformationDTO {
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub code: String,
    pub update_at: i64,
}

// --- Params DTO ---
#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelStatus", js_name = "ModelStatusParamsDTO")]
pub struct ModelStatusParamsDTO {
    pub id: Option<i32>,
    pub model_id: String,
    pub init_params: String, // JSON String
    pub graphic_id: String,
    pub code: String,
    pub r#type: String,
    pub name: String,
    pub status: i32,
    pub actived: i32,
}

impl From<InforModel> for ModelStatusInformationDTO {
    fn from(m: InforModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            code: m.code,
            update_at: m.update_at,
        }
    }
}

impl From<ParamsModel> for ModelStatusParamsDTO {
    fn from(m: ParamsModel) -> Self {
        Self {
            id: Some(m.id),
            model_id: m.model_id,
            init_params: m.init_params,
            graphic_id: m.graphic_id,
            code: m.code,
            r#type: m.r#type,
            name: m.name,
            status: m.status,
            actived: m.actived,
        }
    }
}

/// 内部辅助：将 ParamsModel 转换为平铺后的 JSON 对象 (对应 TS 的 ...checkStringIsObject)
fn transform_params_to_flat_json(m: ParamsModel) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), serde_json::json!(m.graphic_id)); // TS 中 id 取的是 graphicId
    map.insert("graphicId".to_string(), serde_json::json!(m.graphic_id));
    map.insert("name".to_string(), serde_json::json!(m.name));
    map.insert("code".to_string(), serde_json::json!(m.code));
    map.insert("actived".to_string(), serde_json::json!(m.actived));
    map.insert("status".to_string(), serde_json::json!(m.status));
    map.insert("type".to_string(), serde_json::json!(m.r#type));
    map.insert("modelId".to_string(), serde_json::json!(m.model_id));
    map.insert("initParams".to_string(), serde_json::json!(m.init_params));

    // 调用之前写好的工具函数进行“拆箱”并平铺
    unpack_from_storage_handle(map).into()
}

pub async fn get_params_by_material_list(
    model_id: String,
    node_type: String,
) -> Result<Vec<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    // 这里假设 "material" 是字符串，或者使用你的 NodeType::Material.as_ref()
    let objects = ParamsEntity::find()
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::Type.eq(node_type))
        .all(db)
        .await?;

    Ok(objects
        .into_iter()
        .map(transform_params_to_flat_json)
        .collect())
}

pub async fn get_params_by_code_list(
    graphic_id: String,
    model_id: String,
    code: String,
) -> Result<Option<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    // 这里假设 "material" 是字符串，或者使用你的 NodeType::Material.as_ref()
    let objects = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.eq(graphic_id))
        .filter(ParamsColumn::Code.eq(code))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .one(db)
        .await?;

    Ok(objects.map(transform_params_to_flat_json))
}

pub async fn get_all_code_params_by_list(
    graphic_id: String,
    model_id: String,
) -> Result<Vec<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    // 这里假设 "material" 是字符串，或者使用你的 NodeType::Material.as_ref()
    let objects = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.eq(graphic_id))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .all(db)
        .await?;

    Ok(objects
        .into_iter()
        .map(transform_params_to_flat_json)
        .collect())
}

pub async fn get_actived_params_by_code(
    code: String,
    actived: i32,
    model_id: String,
) -> Result<Vec<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    let objects = ParamsEntity::find()
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::Code.eq(code))
        .filter(ParamsColumn::Actived.eq(actived))
        .all(db)
        .await?;

    Ok(objects
        .into_iter()
        .map(transform_params_to_flat_json)
        .collect())
}

/// 获取当前状态数据 (对应 getModelDynamicInformationEntityByIdAndModelId)
pub async fn get_dynamic_info_with_objects(
    code: String,
    model_id: String,
) -> Result<Option<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    // 先查 Info 表 (getModelDynamicInforQueryBuilder)
    let info = InforEntity::find()
        .filter(InforColumn::Code.eq(code.clone()))
        .filter(InforColumn::ModelId.eq(model_id.clone()))
        .one(db)
        .await?;

    if let Some(info_res) = info {
        // 再查 Params 表 (getModelBeginInitParamsEntityQueryBuilder)
        let objects = ParamsEntity::find()
            .filter(ParamsColumn::Code.eq(code))
            .filter(ParamsColumn::ModelId.eq(model_id))
            .all(db)
            .await?;

        // 组装返回结构
        let flat_objects: Vec<serde_json::Value> = objects
            .into_iter()
            .map(transform_params_to_flat_json)
            .collect();

        Ok(Some(serde_json::json!({
            "id": info_res.id,
            "modelId": info_res.model_id,
            "objects": flat_objects
        })))
    } else {
        Ok(None)
    }
}

pub async fn update_node_name(
    model_id: String,
    graphic_id: String,
    name: String,
) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    let res = ParamsEntity::update_many()
        .col_expr(ParamsColumn::Name, Expr::value(name))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::GraphicId.eq(graphic_id))
        .exec(db)
        .await?;

    Ok(res.rows_affected as u32)
}
pub async fn delete_params_by_graphic_id(
    model_id: String,
    graphic_id: String,
) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    let res = ParamsEntity::delete_many()
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::GraphicId.eq(graphic_id))
        .exec(db)
        .await?;

    Ok(res.rows_affected as u32)
}

pub async fn delete_params_where_graphic_id_null(model_id: String) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    let res = ParamsEntity::delete_many()
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::GraphicId.is_null()) // Sea-ORM 处理 Option 字段
        .exec(db)
        .await?;

    Ok(res.rows_affected as u32)
}

/// 内部辅助结构体：用于减少函数参数数量
struct UpsertParamsContext {
    model_id: String,
    graphic_id: String,
    r#type: String,
    actived: i32,
    status: i32,
    name: String,
    init_params: serde_json::Value,
    code: String,
}

async fn upsert_params_by_code(
    ctx: UpsertParamsContext,
    db: &DatabaseTransaction,
) -> Result<bool, DbErr> {
    let init_params_str = if ctx.init_params.is_object() {
        ctx.init_params.to_string()
    } else {
        ctx.init_params.as_str().unwrap_or("{}").to_string()
    };

    let existing = ParamsEntity::find()
        .filter(ParamsColumn::ModelId.eq(ctx.model_id.clone()))
        .filter(ParamsColumn::GraphicId.eq(ctx.graphic_id.clone()))
        .filter(ParamsColumn::Code.eq(ctx.code.clone()))
        .one(db)
        .await?;

    if let Some(model) = existing {
        let mut am: ParamsActiveModel = model.into();
        am.actived = Set(ctx.actived);
        am.status = Set(ctx.status);
        am.name = Set(ctx.name);
        am.init_params = Set(init_params_str);
        am.update(db).await?;
    } else {
        let am = ParamsActiveModel {
            model_id: Set(ctx.model_id),
            graphic_id: Set(ctx.graphic_id),
            code: Set(ctx.code),
            r#type: Set(ctx.r#type),
            name: Set(ctx.name),
            status: Set(ctx.status),
            actived: Set(ctx.actived),
            init_params: Set(init_params_str),
            ..Default::default()
        };
        ParamsEntity::insert(am).exec(db).await?;
    }
    Ok(true)
}

pub async fn add_node_to_all_status_versions(
    model_id: String,
    data: ModelStatusParamsDTO,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    // 1. 获取该模型下的所有状态版本 (InforEntity)
    let all_versions = InforEntity::find()
        .filter(InforColumn::ModelId.eq(model_id.clone()))
        .all(db)
        .await?;

    // 3. 开启事务，统一处理
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            for version in all_versions {
                // 将参数打包进上下文结构体
                let ctx = UpsertParamsContext {
                    model_id: model_id.clone(),
                    graphic_id: data.graphic_id.clone(),
                    r#type: data.r#type.clone(),
                    actived: data.actived, // 修复：去除了 .clone()
                    status: data.status,   // 修复：去除了 .clone()
                    name: data.name.clone(),
                    init_params: serde_json::from_str(&data.init_params).unwrap_or(json!({})),
                    code: version.code,
                };
                upsert_params_by_code(ctx, txn).await?;
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(db_err) => db_err,
        TransactionError::Transaction(db_err) => db_err,
    })?;

    Ok(true)
}

/// 批量同步状态参数 (对应 updateModelStatusParamsEntityByCodeListMsg)
pub async fn batch_sync_params(
    datas: Vec<ModelStatusParamsDTO>,
    code: String,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let graphic_ids: Vec<String> = datas.iter().map(|d| d.graphic_id.clone()).collect();

    // 1. 查找已存在的记录
    let existing = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.is_in(graphic_ids))
        .filter(ParamsColumn::Code.eq(code.clone()))
        .filter(ParamsColumn::ModelId.eq(model_id.clone()))
        .all(db)
        .await?;

    let existing_set: std::collections::HashSet<String> =
        existing.into_iter().map(|m| m.graphic_id).collect();

    // 2. 开启事务处理
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            for item in datas {
                if existing_set.contains(&item.graphic_id) {
                    // 更新
                    ParamsEntity::update_many()
                        .col_expr(ParamsColumn::Actived, Expr::value(item.actived))
                        .col_expr(ParamsColumn::Status, Expr::value(item.status))
                        .col_expr(ParamsColumn::Name, Expr::value(item.name))
                        .col_expr(ParamsColumn::InitParams, Expr::value(item.init_params))
                        .filter(ParamsColumn::GraphicId.eq(item.graphic_id))
                        .filter(ParamsColumn::Code.eq(code.clone()))
                        .filter(ParamsColumn::ModelId.eq(model_id.clone()))
                        .exec(txn)
                        .await?;
                } else {
                    // 插入
                    let am = ParamsActiveModel {
                        model_id: Set(item.model_id),
                        graphic_id: Set(item.graphic_id),
                        code: Set(item.code),
                        r#type: Set(item.r#type),
                        name: Set(item.name),
                        status: Set(item.status),
                        actived: Set(item.actived),
                        init_params: Set(item.init_params),
                        ..Default::default()
                    };
                    ParamsEntity::insert(am).exec(txn).await?;
                }
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(db_err) => db_err,
        TransactionError::Transaction(db_err) => db_err,
    })?;

    Ok(true)
}

/// 更新或者创建状态数据
pub async fn update_or_creat_status_by_infor_and_params(
    data: ModelStatusInformationDTO,
    element_list: Vec<ModelStatusParamsDTO>,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    // let graphic_ids: Vec<String> = datas.iter().map(|d| d.graphic_id.clone()).collect();

    // 1. 查找已存在的记录
    let existing = InforEntity::find()
        .filter(InforColumn::Code.eq(data.code.clone()))
        .filter(InforColumn::ModelId.eq(data.model_id.clone()))
        .count(db)
        .await?;

    if existing > 0 {
        batch_sync_params(element_list, data.code, data.model_id).await?;
    } else {
        let active_models: Vec<ParamsActiveModel> = element_list
            .into_iter()
            .map(|item| ParamsActiveModel {
                model_id: Set(item.model_id),
                graphic_id: Set(item.graphic_id),
                code: Set(item.code),
                r#type: Set(item.r#type),
                name: Set(item.name),
                status: Set(item.status),
                actived: Set(item.actived),
                init_params: Set(item.init_params),
                ..Default::default()
            })
            .collect();
        ParamsEntity::insert_many(active_models).exec(db).await?;
    }

    Ok(true)
}

pub async fn update_active_status_bulk(
    datas: Vec<serde_json::Value>, // 包含 id, name, type, actived 的对象数组
    codes: Vec<String>,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            for item in datas {
                let graphic_id = item["id"].as_str().unwrap_or_default();
                let r#type = item["type"].as_str().unwrap_or_default();
                let actived = item["actived"].as_i64().unwrap_or(0) as i32;

                // 执行更新
                ParamsEntity::update_many()
                    .col_expr(ParamsColumn::Actived, Expr::value(actived))
                    .filter(ParamsColumn::ModelId.eq(model_id.clone()))
                    .filter(ParamsColumn::GraphicId.eq(graphic_id))
                    .filter(ParamsColumn::Type.eq(r#type))
                    .filter(ParamsColumn::Code.is_in(codes.clone()))
                    .exec(txn)
                    .await?;
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| DbErr::Custom(format!("Transaction failed: {:?}", e)))?;

    Ok(true)
}

pub async fn update_dynamic_objects_ex(
    model_id: String,
    code: String,
    res_graphic_element_list: Vec<serde_json::Value>,
) -> Result<bool, DbErr> {
    let mut processed_datas = Vec::new();

    for item in res_graphic_element_list {
        if let serde_json::Value::Object(mut obj_map) = item {
            // 确保传入的对象包含 modelId，以便 pack 函数提取
            obj_map.insert("modelId".to_string(), json!(model_id));

            // 调用你最新的【装箱】工具函数
            let db_map = pack_to_storage_handle(obj_map);

            // 将 Map 转换为 DTO
            // 注意：pack 函数返回的 Map Key 必须与 DTO 字段一一对应
            processed_datas.push(ModelStatusParamsDTO {
                id: None,
                model_id: model_id.clone(),
                graphic_id: db_map
                    .get("graphicId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                code: code.clone(),
                r#type: db_map
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                name: db_map
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                status: db_map.get("status").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                actived: db_map.get("actived").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
                init_params: db_map
                    .get("initParams")
                    .and_then(|v| v.as_str())
                    .unwrap_or("{}")
                    .to_string(),
            });
        }
    }

    // 调用现有的批量同步逻辑
    batch_sync_params(processed_datas, code, model_id).await
}

pub async fn get_params_by_ids_and_code(
    graphic_ids: Vec<String>,
    model_id: String,
    code: Option<String>,
) -> Result<Vec<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;
    let target_code = code.unwrap_or_else(|| "Normal".to_string());

    let results = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.is_in(graphic_ids))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .filter(ParamsColumn::Code.eq(target_code))
        .all(db)
        .await?;

    let list = results
        .into_iter()
        .map(|m| {
            // 1. 将数据库 Model 转为 Map
            let mut map = serde_json::Map::new();
            map.insert("graphicId".to_string(), json!(m.graphic_id));
            map.insert("name".to_string(), json!(m.name));
            map.insert("actived".to_string(), json!(m.actived));
            map.insert("status".to_string(), json!(m.status));
            map.insert("type".to_string(), json!(m.r#type));
            map.insert("modelId".to_string(), json!(m.model_id));
            map.insert("initParams".to_string(), json!(m.init_params));

            // 2. 调用【拆箱】工具函数：解析字符串并平铺字段
            let mut unpacked_map = unpack_from_storage_handle(map);

            // 3. 特殊处理：TS 原版要求返回的 id 字段对应 graphicId
            if let Some(gid) = unpacked_map.get("graphicId") {
                unpacked_map.insert("id".to_string(), gid.clone());
            }

            serde_json::Value::Object(unpacked_map)
        })
        .collect();

    Ok(list)
}

// 更新状态列表名称
pub async fn update_info_name(model_id: String, code: String, name: String) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    let res = InforEntity::update_many()
        .col_expr(InforColumn::Name, Expr::value(name))
        .filter(InforColumn::ModelId.eq(model_id))
        .filter(InforColumn::Code.eq(code))
        .exec(db)
        .await?;

    Ok(res.rows_affected as u32)
}

/// 获取最新更新的状态 (对应 getLatestModelDynamicInformationEntity)
pub async fn get_latest_status(
    model_id: String,
) -> Result<Option<ModelStatusInformationDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = InforEntity::find()
        .filter(InforColumn::ModelId.eq(model_id))
        .order_by_desc(InforColumn::UpdateAt)
        .one(db)
        .await?;

    Ok(res.map(ModelStatusInformationDTO::from))
}
// 更新状态列表最新的更新时间
pub async fn update_info_update_at(model_id: String, code: String) -> Result<u32, DbErr> {
    let db = get_business_db().await?;

    let res = InforEntity::update_many()
        .col_expr(
            InforColumn::UpdateAt,
            Expr::value(Utc::now().timestamp_millis()),
        )
        .filter(InforColumn::ModelId.eq(model_id))
        .filter(InforColumn::Code.eq(code))
        .exec(db)
        .await?;

    Ok(res.rows_affected as u32)
}

/// 级联删除状态 (删除 Info 表的同时删除 Params 表)
pub async fn delete_status_cascade(codes: Vec<String>, model_id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            ParamsEntity::delete_many()
                .filter(ParamsColumn::Code.is_in(codes.clone()))
                .filter(ParamsColumn::ModelId.eq(model_id.clone()))
                .exec(txn)
                .await?;

            InforEntity::delete_many()
                .filter(InforColumn::Code.is_in(codes))
                .filter(InforColumn::ModelId.eq(model_id))
                .exec(txn)
                .await?;
            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(db_err) => db_err,
        TransactionError::Transaction(db_err) => db_err,
    })?;

    Ok(true)
}

pub async fn update_all_graphic_model(update_data: Vec<serde_json::Value>) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    for item in update_data {
        let id = item["id"].as_i64().unwrap_or(0) as i32;
        let model_id = item["modelId"].as_str().unwrap_or_default();
        let init_params = &item["initParams"];

        // 将对象转为字符串存储
        let init_params_str = if init_params.is_object() {
            init_params.to_string()
        } else {
            init_params.as_str().unwrap_or("{}").to_string()
        };

        ParamsEntity::update_many()
            .col_expr(ParamsColumn::InitParams, Expr::value(init_params_str))
            .filter(ParamsColumn::Id.eq(id))
            .filter(ParamsColumn::ModelId.eq(model_id))
            .exec(db)
            .await?;
    }

    Ok(true)
}

pub async fn get_params_by_graphic_id_all_versions(
    graphic_id: String,
    model_id: String,
) -> Result<Vec<ModelStatusParamsDTO>, DbErr> {
    let db = get_business_db().await?;

    let results = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.eq(graphic_id))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .all(db)
        .await?;

    // 转换为 Value 列表（如果需要平铺，可在此调用 unpack_from_storage_handle）
    Ok(results
        .into_iter()
        .map(ModelStatusParamsDTO::from)
        .collect())
}

pub async fn get_status_params_by_ids_and_all_code(
    graphic_ids: Vec<String>,
    r#type: String,
    model_id: String,
) -> Result<Vec<serde_json::Value>, DbErr> {
    let db = get_business_db().await?;

    let results = ParamsEntity::find()
        .filter(ParamsColumn::GraphicId.is_in(graphic_ids))
        .filter(ParamsColumn::Type.eq(r#type))
        .filter(ParamsColumn::ModelId.eq(model_id))
        .all(db)
        .await?;

    let list = results
        .into_iter()
        .map(|m| {
            // 1. 将 Model 转为 Map
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), json!(m.id));
            map.insert("graphicId".to_string(), json!(m.graphic_id));
            map.insert("name".to_string(), json!(m.name));
            map.insert("type".to_string(), json!(m.r#type));
            map.insert("status".to_string(), json!(m.status));
            map.insert("actived".to_string(), json!(m.actived));
            map.insert("code".to_string(), json!(m.code));
            map.insert("modelId".to_string(), json!(m.model_id));
            map.insert("initParams".to_string(), json!(m.init_params));

            // 2. 还原平铺字段
            serde_json::Value::Object(unpack_from_storage_handle(map))
        })
        .collect();

    Ok(list)
}
pub async fn insert_all_params_redo(entities: Vec<serde_json::Value>) -> Result<(), DbErr> {
    let db = get_business_db().await?;

    let mut active_models = Vec::new();
    for item in entities {
        active_models.push(ParamsActiveModel {
            model_id: Set(item["modelId"].as_str().unwrap_or_default().to_string()),
            graphic_id: Set(item["graphicId"].as_str().unwrap_or_default().to_string()),
            code: Set(item["code"].as_str().unwrap_or_default().to_string()),
            r#type: Set(item["type"].as_str().unwrap_or_default().to_string()),
            name: Set(item["name"].as_str().unwrap_or_default().to_string()),
            status: Set(item["status"].as_i64().unwrap_or(0) as i32),
            actived: Set(item["actived"].as_i64().unwrap_or(1) as i32),
            init_params: Set(item["initParams"].as_str().unwrap_or("{}").to_string()),
            ..Default::default()
        });
    }

    if !active_models.is_empty() {
        ParamsEntity::insert_many(active_models).exec(db).await?;
    }

    Ok(())
}
pub async fn update_params_by_type_only_init_params(
    update_datas: Vec<serde_json::Value>,
    r#type: String,
    model_id: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;

    if update_datas.is_empty() {
        return Ok(true);
    }

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            for item in update_datas {
                let graphic_id = item["graphicId"].as_str().unwrap_or_default();
                let code = item["code"].as_str().unwrap_or_default();
                let status = item["status"].as_i64().unwrap_or(0) as i32;
                let init_params = &item["initParams"];

                let init_params_str = if init_params.is_object() {
                    init_params.to_string()
                } else {
                    init_params.as_str().unwrap_or("{}").to_string()
                };

                ParamsEntity::update_many()
                    .col_expr(ParamsColumn::Status, Expr::value(status))
                    .col_expr(ParamsColumn::InitParams, Expr::value(init_params_str))
                    .filter(ParamsColumn::GraphicId.eq(graphic_id))
                    .filter(ParamsColumn::ModelId.eq(model_id.clone()))
                    .filter(ParamsColumn::Code.eq(code))
                    .filter(ParamsColumn::Type.eq(r#type.clone()))
                    .exec(txn)
                    .await?;
            }
            Ok(())
        })
    })
    .await
    .map_err(|e| DbErr::Custom(format!("Transaction failed: {:?}", e)))?;

    Ok(true)
}
