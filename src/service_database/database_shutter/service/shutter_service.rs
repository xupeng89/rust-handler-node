use crate::service_database::database_shutter::db_shutter_connection::get_shutter_db;
use chrono::Utc;
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, IntoActiveModel,
    QueryFilter, QuerySelect, Set, TransactionError, TransactionTrait, entity::prelude::*,
};
use serde::{Deserialize, Serialize};
use zstd::{decode_all, encode_all};
// 引入两个实体
use crate::service_database::database_shutter::entity::model_shutter_data_entity::{
    ActiveModel as DataActiveModel, Column as DataColumn, Entity as DataEntity,
};
use crate::service_database::database_shutter::entity::model_shutter_entity::{
    ActiveModel as MainActiveModel, Column as MainColumn, Entity as MainEntity,
};

// ======================================
// 工具函数：压缩
// ======================================
fn compress_data(data: &str) -> Vec<u8> {
    encode_all(data.as_bytes(), 3).unwrap_or_default()
}

fn decompress_data(data: &[u8]) -> String {
    let decoded = decode_all(data).unwrap_or_default();
    String::from_utf8(decoded).unwrap_or_default()
}

// ======================================
// DTO 定义 (保持与 NAPI 兼容)
// ======================================
#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(object, namespace = "shutterHandle")]
pub struct FullShutterModel {
    pub id: String,
    pub name: String,
    pub index_num: i32,
    pub update_at: String,
    pub objects: String, // 对外依然表现为 String
    pub sysvars: String,
    pub model_id: String,
    pub user_name: Option<String>,
    pub type_num: Option<i32>,
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

// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[napi(object, namespace = "shutterHandle")]
// pub struct FullShutterData {
//     pub id: String,
//     pub name: String,
//     pub index_num: i32,
//     pub model_id: String,
//     pub objects: String,
//     pub sysvars: String,
//     pub update_at: String,
//     pub base_state_code: String,
//     pub user_name: Option<String>,
//     pub state_index: Option<i32>,
//     pub state_desc: Option<String>,
//     pub type_num: Option<i32>,
// }

// ======================================
// 数据库操作改造
// ======================================

/// 获取列表 (仅查主表，极快)
pub async fn get_all_model_shutter_entity_list(
    model_id: String,
) -> Result<Vec<ShutterListItem>, DbErr> {
    let db = get_shutter_db().await?;
    let results = MainEntity::find()
        .select_only()
        .columns([
            MainColumn::Id,
            MainColumn::Name,
            MainColumn::UpdateAt,
            MainColumn::IndexNum,
            MainColumn::BaseStateCode,
        ])
        .filter(MainColumn::ModelId.eq(model_id))
        .into_model::<ShutterListItem>()
        .all(db)
        .await?;
    Ok(results)
}

/// 获取详情 (JOIN + 解压缩)
pub async fn get_model_shutter_entity_by_id(
    id: String,
    model_id: String,
) -> Result<Option<FullShutterModel>, DbErr> {
    let db = get_shutter_db().await?;

    // 使用 find_also_related 一次性查出主表和压缩数据
    let result = MainEntity::find_by_id(id.clone())
        .filter(MainColumn::ModelId.eq(model_id))
        .find_also_related(DataEntity)
        .one(db)
        .await?;

    if let Some((main, Some(data))) = result {
        Ok(Some(FullShutterModel {
            id: main.id,
            name: main.name,
            index_num: main.index_num,
            model_id: main.model_id,
            objects: decompress_data(&data.objects), // 解压
            sysvars: decompress_data(&data.sysvars), // 解压
            update_at: main.update_at,
            base_state_code: main.base_state_code,
            user_name: main.user_name,
            state_index: main.state_index,
            state_desc: main.state_desc,
            type_num: main.type_num,
        }))
    } else {
        Ok(None)
    }
}

/// 插入或更新逻辑 (带事务和压缩)
pub async fn insert_model_shutter_entity(
    data: FullShutterModel,
    model_id: String,
) -> Result<i32, DbErr> {
    let db = get_shutter_db().await?;

    db.transaction::<_, i32, DbErr>(|txn| {
        Box::pin(async move {
            let existing = MainEntity::find()
                .filter(MainColumn::ModelId.eq(model_id))
                .filter(MainColumn::IndexNum.eq(data.index_num))
                .one(txn)
                .await?;

            if let Some(record) = existing {
                // 1. 更新主表
                let mut active_main: MainActiveModel = record.into_active_model();
                active_main.base_state_code = Set(data.base_state_code);
                active_main.update_at = Set(data.update_at);
                let active_main_id = active_main.id.clone();
                active_main.update(txn).await?;

                // 2. 更新数据表
                let active_data = DataActiveModel {
                    id: Set(active_main_id.unwrap()),
                    objects: Set(compress_data(&data.objects)),
                    sysvars: Set(compress_data(&data.sysvars)),
                };
                DataEntity::update(active_data).exec(txn).await?;
                Ok(data.index_num)
            } else {
                // 1. 插入主表
                let active_main = MainActiveModel {
                    id: Set(data.id.clone()),
                    model_id: Set(data.model_id),
                    name: Set(data.name),
                    index_num: Set(data.index_num),
                    update_at: Set(data.update_at),
                    base_state_code: Set(data.base_state_code),
                    user_name: Set(data.user_name),
                    state_index: Set(data.state_index),
                    state_desc: Set(data.state_desc),
                    type_num: Set(data.type_num),
                };
                active_main.insert(txn).await?;

                // 2. 插入数据表
                let active_data = DataActiveModel {
                    id: Set(data.id),
                    objects: Set(compress_data(&data.objects)),
                    sysvars: Set(compress_data(&data.sysvars)),
                };
                DataEntity::insert(active_data).exec(txn).await?;
                Ok(data.index_num)
            }
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => e,
        TransactionError::Transaction(e) => e,
    })
}

/// 仅插入 (insertModelShutterEntityOnly)
pub async fn insert_model_shutter_entity_only(data: FullShutterModel) -> Result<(), DbErr> {
    let db = get_shutter_db().await?;

    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let active_main = MainActiveModel {
                id: Set(data.id.clone()),
                model_id: Set(data.model_id),
                name: Set(data.name),
                index_num: Set(data.index_num),
                update_at: Set(data.update_at),
                base_state_code: Set(data.base_state_code),
                user_name: Set(data.user_name),
                state_index: Set(data.state_index),
                state_desc: Set(data.state_desc),
                type_num: Set(data.type_num),
            };
            active_main.insert(txn).await?;

            let active_data = DataActiveModel {
                id: Set(data.id),
                objects: Set(compress_data(&data.objects)),
                sysvars: Set(compress_data(&data.sysvars)),
            };
            DataEntity::insert(active_data).exec(txn).await?;
            Ok(())
        })
    })
    .await
    .map_err(|e| match e {
        TransactionError::Connection(e) => e,
        TransactionError::Transaction(e) => e,
    })
}

/// 删除快照 (需同时删除两表数据)
pub async fn delete_model_shutter_entity(id: String, model_id: String) -> Result<u64, DbErr> {
    let db = get_shutter_db().await?;

    // 注意：如果在数据库设置了级联删除（ON DELETE CASCADE），只需删除主表。
    // 否则需要在事务中删除两表。
    let result = MainEntity::delete_many()
        .filter(MainColumn::Id.eq(id.clone()))
        .filter(MainColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;

    // 手动删除数据表内容
    DataEntity::delete_by_id(id).exec(db).await?;

    Ok(result.rows_affected)
}

/// 根据 ID 更新部分信息 (适配分表+压缩架构)
pub async fn update_model_shutter_entity_by_id_only(
    id: String,
    objects: String,
    sysvars: String,
    base_state_code: String,
) -> Result<u64, DbErr> {
    let db = get_shutter_db().await?;

    // 涉及两个表的更新，必须使用事务
    let rows_affected = db
        .transaction::<_, u64, DbErr>(|txn| {
            let id_clone = id.clone();
            Box::pin(async move {
                // 1. 更新主表 (model_shutter_entity) 的状态码
                let main_res = MainEntity::update_many()
                    .col_expr(MainColumn::BaseStateCode, Expr::value(base_state_code))
                    .col_expr(MainColumn::UpdateAt, Expr::value(Utc::now().to_rfc3339()))
                    .filter(MainColumn::Id.eq(id_clone.clone()))
                    .exec(txn)
                    .await?;

                // 如果主表没找到记录，直接返回 0
                if main_res.rows_affected == 0 {
                    return Ok(0);
                }

                // 2. 更新数据表 (model_shutter_data) 的大字段 (进行 zstd 压缩)
                DataEntity::update_many()
                    .col_expr(
                        DataColumn::Objects,
                        Expr::value(compress_data(&objects)), // 压缩 String -> Vec<u8>
                    )
                    .col_expr(
                        DataColumn::Sysvars,
                        Expr::value(compress_data(&sysvars)), // 压缩 String -> Vec<u8>
                    )
                    .filter(DataColumn::Id.eq(id_clone))
                    .exec(txn)
                    .await?;

                // 返回受影响的行数（通常为 1）
                Ok(main_res.rows_affected)
            })
        })
        .await
        .map_err(|e| match e {
            TransactionError::Connection(e) => e,
            TransactionError::Transaction(e) => e,
        })?;

    Ok(rows_affected)
}
