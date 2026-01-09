use crate::{
    service_database::database_auto_shutter::db_auto_shutter_connection::get_auto_shutter_db,
    tool_handle::time_tool::integer_to_string,
};
use chrono::Utc;
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, IntoActiveModel, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionError, TransactionTrait,
};
use serde::{Deserialize, Serialize};

// 引入拆分后的两个实体
use crate::service_database::database_auto_shutter::entity::model_auto_shutter_data_entity::{
    ActiveModel as DataActiveModel, Entity as DataEntity,
};
use crate::service_database::database_auto_shutter::entity::model_auto_shutter_entity::{
    ActiveModel as MainActiveModel, Column as MainColumn, Entity as MainEntity,
};

// ======================================
// 内部工具：压缩与解压
// ======================================
use zstd::{decode_all, encode_all};
fn compress_data(data: &str) -> Vec<u8> {
    // 压缩级别 3 是性能和压缩率的最佳平衡点
    encode_all(data.as_bytes(), 3).unwrap_or_default()
}

fn decompress_data(data: &[u8]) -> String {
    let decoded = decode_all(data).unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_default()
}

// ======================================
// DTO 定义
// ======================================

#[derive(Clone, Debug, Deserialize, Serialize)]
#[napi(object, namespace = "autoShutter")]
pub struct AutoShutterData {
    pub objects: String,
    pub sysvars: String,
    pub sim_time: String,
    pub base_state_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct FullCacheData {
    pub id: i32,
    pub model_id: String,
    pub objects: String,
    pub sysvars: String,
    pub update_at: String,
    pub sim_time: String,
    pub base_state_code: String,
    pub user_name: Option<String>,
    pub state_index: Option<i32>,
    pub state_desc: Option<String>,
}

#[derive(Clone, Debug, Serialize, FromQueryResult)]
#[napi(object, namespace = "autoShutter")]
pub struct AutoShutterListItem {
    pub id: i32,
    pub update_at: String,
    pub sim_time: String,
    pub base_state_code: String,
}

#[derive(Clone, Debug, FromQueryResult)]
struct AutoShutterSelectModel {
    pub id: i32,
    pub update_at: i64,
    pub sim_time: String,
    pub base_state_code: String,
}

// ======================================
// 业务逻辑实现
// ======================================

/// 查询当前快照数量 (仅查主表，极快)
pub async fn get_model_auto_shutter_entity_count(model_id: String) -> Result<u32, DbErr> {
    let db = get_auto_shutter_db().await?;
    let result_count = MainEntity::find()
        .filter(MainColumn::ModelId.eq(model_id))
        .count(db)
        .await?;
    Ok(result_count as u32)
}

/// 插入一个数据 (包含压缩处理)
pub async fn read_one_model_auto_shutter_entity_cache(
    data: AutoShutterData,
    model_id: String,
) -> Result<i32, DbErr> {
    let db = get_auto_shutter_db().await?;

    db.transaction::<_, i32, DbErr>(|txn| {
        Box::pin(async move {
            // 1. 插入主表元数据
            let main_item = MainActiveModel {
                model_id: Set(model_id),
                update_at: Set(Utc::now().timestamp_millis()),
                sim_time: Set(data.sim_time),
                base_state_code: Set(data.base_state_code),
                ..Default::default()
            };
            let main_res = MainEntity::insert(main_item).exec(txn).await?;
            let new_id = main_res.last_insert_id;

            // 2. 插入大数据表 (压缩后存入)
            let data_item = DataActiveModel {
                id: Set(new_id),
                objects: Set(compress_data(&data.objects)),
                sysvars: Set(compress_data(&data.sysvars)),
            };
            DataEntity::insert(data_item).exec(txn).await?;

            Ok(new_id)
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => e,
        TransactionError::Transaction(e) => e,
    })
}

/// 更新自动快照 (高并发安全 + 二进制压缩)
pub async fn update_model_auto_shutter_entity_cache(
    data: AutoShutterData,
    model_id: String,
) -> Result<i32, DbErr> {
    let db = get_auto_shutter_db().await?;

    let updated_id = db
        .transaction::<_, i32, DbErr>(|txn| {
            Box::pin(async move {
                // 找到最旧的一条记录进行覆盖（轮转逻辑）
                let target_record = MainEntity::find()
                    .filter(MainColumn::ModelId.eq(model_id))
                    .order_by_asc(MainColumn::UpdateAt)
                    .lock_exclusive()
                    .one(txn)
                    .await?;

                if let Some(record) = target_record {
                    let tid = record.id;

                    // 1. 更新主表
                    let mut active_main = record.into_active_model();
                    active_main.update_at = Set(Utc::now().timestamp_millis());
                    active_main.sim_time = Set(data.sim_time);
                    active_main.base_state_code = Set(data.base_state_code);
                    active_main.update(txn).await?;

                    // 2. 更新数据表 (压缩)
                    let active_data = DataActiveModel {
                        id: Set(tid),
                        objects: Set(compress_data(&data.objects)),
                        sysvars: Set(compress_data(&data.sysvars)),
                    };
                    DataEntity::update(active_data).exec(txn).await?;

                    Ok(tid)
                } else {
                    Err(DbErr::RecordNotFound("未找到快照记录".to_string()))
                }
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(e) => e,
            TransactionError::Transaction(e) => e,
        })?;

    Ok(updated_id)
}

/// 获取快照列表 (仅查主表，不加载大数据字段，速度极快)
pub async fn get_all_model_auto_shutter_entity_list_cache(
    order_flag: String,
    auto_count: u32,
    model_id: String,
) -> Result<Vec<AutoShutterListItem>, DbErr> {
    let db = get_auto_shutter_db().await?;
    let order = if order_flag.to_uppercase() == "ASC" {
        Order::Asc
    } else {
        Order::Desc
    };

    let result = MainEntity::find()
        .select_only()
        .column(MainColumn::Id)
        .column(MainColumn::UpdateAt)
        .column(MainColumn::SimTime)
        .column(MainColumn::BaseStateCode)
        .filter(MainColumn::ModelId.eq(model_id))
        .order_by(MainColumn::UpdateAt, order)
        .limit(auto_count as u64)
        .into_model::<AutoShutterSelectModel>()
        .all(db)
        .await?;

    Ok(result
        .into_iter()
        .map(|ele| AutoShutterListItem {
            id: ele.id,
            update_at: integer_to_string(ele.update_at),
            sim_time: ele.sim_time,
            base_state_code: ele.base_state_code,
        })
        .collect())
}

/// 获取单个详情 (JOIN 查询 + 解压缩)
pub async fn get_model_auto_shutter_entity_by_id_cache(
    id: i32,
    model_id: String,
) -> Result<FullCacheData, DbErr> {
    let db = get_auto_shutter_db().await?;

    // 使用 find_also_related 同时查出主表和压缩数据表
    let result = MainEntity::find_by_id(id)
        .filter(MainColumn::ModelId.eq(model_id))
        .find_also_related(DataEntity)
        .one(db)
        .await?;

    match result {
        Some((main, Some(data))) => {
            Ok(FullCacheData {
                id: main.id,
                model_id: main.model_id,
                objects: decompress_data(&data.objects), // 解压
                sysvars: decompress_data(&data.sysvars), // 解压
                update_at: integer_to_string(main.update_at),
                sim_time: main.sim_time,
                base_state_code: main.base_state_code,
                user_name: main.user_name,
                state_index: main.state_index,
                state_desc: main.state_desc,
            })
        }
        _ => Err(DbErr::RecordNotFound(format!("ID {} not found", id))),
    }
}
