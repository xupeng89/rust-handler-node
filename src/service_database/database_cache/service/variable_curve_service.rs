use chrono::Utc;
use sea_orm::{entity::*, query::*, ActiveValue::Set, DbErr, InsertResult};
use serde::{Deserialize, Serialize};

// 假设 Entity 定义位于此路径下
use crate::service_database::database_cache::entity::model_variable_curve_entity_cache::{
    ActiveModel, Column, Entity, Model,
};
// 导入公共数据库连接函数
use crate::service_database::database_cache::db_cache_connection::get_cache_db;
use napi_derive::napi;
// 对应输入的 datasets 结构
#[derive(Debug, Serialize, Deserialize)]
#[napi(object, namespace = "variableCurveHandle")]
pub struct DatasetItem {
    pub id: String,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "variableCurveHandle")]
pub struct CurveModel {
    pub id: i32,
    pub sim_time: String,
    pub datasets: String,
    pub create_at: String,
    pub model_id: String,
    pub config_id: String,
}

impl From<Model> for CurveModel {
    fn from(model: Model) -> Self {
        CurveModel {
            id: model.id,
            sim_time: model.sim_time,
            datasets: model.datasets,
            create_at: model.create_at,
            config_id: model.config_id, // 假设字段名匹配
            model_id: model.model_id,
        }
    }
}

// =========================================================================
// Service Functions
// =========================================================================

/// 插入变量曲线信息
/// 对应 TypeScript: insertModelVariableCurveEntityCache
pub async fn insert_model_variable_curve_entity_cache(
    datasets: Vec<DatasetItem>,
    model_id: String,
    config_id: String,
    sim_time: f64,
) -> Result<InsertResult<ActiveModel>, DbErr> {
    let db = get_cache_db().await?;
    // 1. 获取当前时间戳 (毫秒)
    let current_time = Utc::now().timestamp_millis().to_string();

    // 2. 序列化 datasets 为 JSON 字符串 (对应 checkAnyTypeIsObjectToString)
    let datasets_str = serde_json::to_string(&datasets)
        .map_err(|e| DbErr::Custom(format!("Datasets serialization error: {}", e)))?;

    // 3. 创建 ActiveModel
    let new_entry = ActiveModel {
        // ID 是自增的，不需要设置
        datasets: Set(datasets_str),
        model_id: Set(model_id),
        config_id: Set(config_id),
        // 数据库字段 sim_time 和 ceate_at 是 String 类型，需要转换
        sim_time: Set(sim_time.to_string()),
        create_at: Set(current_time),
        ..Default::default()
    };

    // 4. 执行插入
    Entity::insert(new_entry).exec(db).await
}

// ---

/// 根据模型 ID 和配置 ID 查询所有曲线数据，并按创建时间排序
/// 对应 TypeScript: getModelVariableCurveEntityCacheByFilterTime
pub async fn get_model_variable_curve_entity_cache_by_filter_time(
    model_id: String,
    config_id: String,
) -> Result<Vec<CurveModel>, DbErr> {
    let db = get_cache_db().await?;
    // 组合查询条件
    let filter = Condition::all()
        .add(Column::ModelId.eq(model_id))
        .add(Column::ConfigId.eq(config_id));

    let result = Entity::find()
        .filter(filter)
        // 对应 TypeScript 中的 orderBy("ceateAt", "ASC")
        .order_by_asc(Column::CreateAt)
        .all(db)
        .await?;

    let result_data: Vec<CurveModel> = result.into_iter().map(CurveModel::from).collect();
    Ok(result_data)
}

// ---

/// 根据模型 ID、配置 ID 和时间间隔进行过滤查询 (包含内存逻辑过滤)
/// 对应 TypeScript: getModelVariableCurveEntityCacheByFilterCount
pub async fn get_model_variable_curve_entity_cache_by_filter_count(
    model_id: String,
    config_id: String,
    filter_count: i32, // 时间间隔阈值
) -> Result<Vec<CurveModel>, DbErr> {
    // 1. 获取所有匹配的数据 (复用上面的查询逻辑)
    let results = get_model_variable_curve_entity_cache_by_filter_time(model_id, config_id).await?;

    if results.is_empty() {
        return Ok(vec![]);
    }

    // 2. 内存过滤逻辑
    let mut filtered: Vec<CurveModel> = Vec::new();

    // 保留第一个数据
    filtered.push(results[0].clone());

    // 遍历后续数据
    for current in results.iter().skip(1) {
        // 获取上一个保留的数据
        let prev_item = filtered.last().unwrap();

        // 解析 sim_time (数据库存的是 String，业务逻辑需要 f64)
        // 使用 unwrap_or(0.0) 避免解析失败导致程序崩溃
        let prev_sim: i32 = prev_item.sim_time.parse().unwrap_or(0);
        let current_sim: i32 = current.sim_time.parse().unwrap_or(0);

        // 计算时间差
        let time_gap = current_sim - prev_sim;

        // 如果时间差大于等于阈值，则保留当前数据
        if time_gap >= filter_count {
            filtered.push(current.clone());
        }
    }

    Ok(filtered)
}
