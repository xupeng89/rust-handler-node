// use crate::service_database::database_cache::db_cache_connection::get_cache_db;
// use chrono::{Duration, Utc};
// use napi_derive::napi;
// use sea_orm::{
//     ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, FromQueryResult, IntoActiveModel, Order,
//     PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionError, TransactionTrait,
// };
// use serde::{Deserialize, Serialize};

// // ======================================
// // 引入实体定义
// // ======================================
// // 1. 缓存表实体
// use crate::service_database::database_cache::entity::model_auto_shutter_entity_cache::{
//     ActiveModel as AutoShutterCacheActiveModel, Column as AutoShutterCacheColumn,
//     Entity as AutoShutterCacheEntity, Model as AutoShutterCacheModel,
// };

// use crate::tool_handle::time_tool::{millis_to_naive_dt_utc, naive_dt_utc_to_millis};

// // ======================================
// // DTO 定义 (已调整，新增 FullCacheData 用于数据同步)
// // ======================================

// // [Input/Update DTO] 不含 ID，用于插入或更新
// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[napi(object, namespace = "autoShutterCache")]
// pub struct AutoShutterData {
//     pub objects: String,
//     pub sysvars: String,
//     pub sim_time: String,
//     pub base_state_code: String,
// }

// // [Output DTO - Full Model] 包含所有数据库字段，用于 read/sync 的数据传输
// #[derive(Clone, Debug, Serialize, Deserialize)]
// #[napi(object)]
// pub struct FullCacheData {
//     pub id: i32,
//     pub model_id: String,
//     pub objects: String,
//     pub sysvars: String,
//     pub update_at: String,
//     pub sim_time: String,
//     pub base_state_code: String,
//     pub user_name: Option<String>,
//     pub state_index: Option<i32>,
//     pub state_desc: Option<String>,
// }

// // 读取完整数据
// impl From<AutoShutterCacheModel> for FullCacheData {
//     fn from(ele: AutoShutterCacheModel) -> Self {
//         FullCacheData {
//             id: ele.id,
//             model_id: ele.model_id,
//             objects: ele.objects,
//             sysvars: ele.sysvars,
//             update_at: naive_dt_utc_to_millis(ele.update_at),
//             sim_time: ele.sim_time,
//             base_state_code: ele.base_state_code,
//             user_name: ele.user_name,
//             state_index: ele.state_index,
//             state_desc: ele.state_desc,
//         }
//     }
// }

// // 查询数据给列表使用
// // [Output - List Item] 列表查询只返回部分字段 (包含 ID)
// #[derive(Clone, Debug, Serialize, FromQueryResult)]
// #[napi(object, namespace = "autoShutterCache")]
// pub struct AutoShutterListItem {
//     pub id: i32,
//     pub update_at: String,
//     pub sim_time: String,
//     pub base_state_code: String,
// }

// // 读取完整数据
// impl From<AutoShutterCacheModel> for AutoShutterListItem {
//     fn from(ele: AutoShutterCacheModel) -> Self {
//         AutoShutterListItem {
//             id: ele.id,
//             update_at: (ele.update_at + Duration::hours(8))
//                 .format("%Y-%m-%d %H:%M:%S.%3f")
//                 .to_string(),
//             sim_time: ele.sim_time,
//             base_state_code: ele.base_state_code,
//         }
//     }
// }

// /// 查询当前快照数量
// pub async fn get_model_auto_shutter_entity_count(model_id: String) -> Result<u32, DbErr> {
//     let db = get_cache_db().await?;
//     // 1. 获取所有缓存数据
//     let result_count: u64 = AutoShutterCacheEntity::find()
//         .filter(AutoShutterCacheColumn::ModelId.eq(model_id))
//         .count(db)
//         .await?;

//     Ok(result_count as u32)
// }

// /// 更新自动快照 (高并发安全，保持不变)
// pub async fn update_model_auto_shutter_entity_cache(
//     data: AutoShutterData,
//     model_id: String,
// ) -> Result<i32, DbErr> {
//     // 返回值从 Result<u32, DbErr> 改为 Result<i32, DbErr>（i32 对应 id 类型）
//     let db = get_cache_db().await?;

//     // 事务中捕获更新记录的 ID
//     let updated_id = db
//         .transaction::<_, i32, DbErr>(|txn| {
//             // 事务返回值改为 i32（ID 类型）
//             Box::pin(async move {
//                 let target_record = AutoShutterCacheEntity::find()
//                     .filter(AutoShutterCacheColumn::ModelId.eq(model_id))
//                     .order_by_asc(AutoShutterCacheColumn::UpdateAt)
//                     .lock_exclusive() // 排他锁，避免并发更新冲突
//                     .one(txn)
//                     .await?;

//                 if let Some(record) = target_record {
//                     let updated_id = record.id; // 捕获要更新记录的 ID
//                     let mut active: AutoShutterCacheActiveModel = record.into_active_model();

//                     // 更新字段（保持原逻辑）
//                     active.objects = Set(data.objects);
//                     active.sysvars = Set(data.sysvars);
//                     active.update_at = Set(Utc::now().naive_utc());
//                     active.sim_time = Set(data.sim_time);
//                     active.base_state_code = Set(data.base_state_code);

//                     active.update(txn).await?; // 执行更新
//                     Ok(updated_id) // 事务成功，返回捕获的 ID
//                 } else {
//                     // 若未找到匹配记录，返回自定义错误（或根据业务返回 0/None，这里推荐返回错误）
//                     Err(DbErr::RecordNotFound(
//                         "未找到要更新的自动快门缓存记录".to_string(),
//                     ))
//                 }
//             })
//         })
//         .await
//         .map_err(|e| match e {
//             TransactionError::Connection(e) => e,
//             TransactionError::Transaction(e) => e,
//         })?;

//     Ok(updated_id) // 函数返回更新的 ID
// }

// /// 插入一个数据到缓存数据库
// pub async fn read_one_model_auto_shutter_entity_cache(
//     data: AutoShutterData,
//     model_id: String,
// ) -> Result<i32, DbErr> {
//     let db = get_cache_db().await?;

//     let new_item = AutoShutterCacheActiveModel {
//         model_id: Set(model_id),
//         objects: Set(data.objects),
//         sysvars: Set(data.sysvars),
//         update_at: Set(Utc::now().naive_utc()),
//         sim_time: Set(data.sim_time),
//         base_state_code: Set(data.base_state_code),
//         ..Default::default()
//     };
//     // 插入缓存表
//     let result = AutoShutterCacheEntity::insert(new_item).exec(db).await?;

//     Ok(result.last_insert_id)
// }

// /// 批量插入数据到缓存数据库
// /// TS: readModelAutoShutterEntityCache (现改为接受数据作为参数)
// pub async fn read_model_auto_shutter_entity_cache(data: Vec<FullCacheData>) -> Result<(), DbErr> {
//     let db = get_cache_db().await?;

//     // 转换为 ActiveModel
//     let cache_inserts: Vec<AutoShutterCacheActiveModel> = data
//         .into_iter()
//         .map(|d| AutoShutterCacheActiveModel {
//             id: Set(d.id),
//             model_id: Set(d.model_id),
//             objects: Set(d.objects),
//             sysvars: Set(d.sysvars),
//             update_at: Set(millis_to_naive_dt_utc(d.update_at)),
//             sim_time: Set(d.sim_time),
//             base_state_code: Set(d.base_state_code),
//             ..Default::default()
//         })
//         .collect();

//     // 批量插入缓存表
//     if !cache_inserts.is_empty() {
//         AutoShutterCacheEntity::insert_many(cache_inserts)
//             .exec(db)
//             .await?;
//     }

//     Ok(())
// }

// /// 查询缓存数据并返回 (供外部同步到持久化存储)
// /// TS: updateAllModelAutoShutterEntityCache (现改为返回参数)
// pub async fn get_all_model_auto_shutter_entity_cache() -> Result<Vec<FullCacheData>, DbErr> {
//     let db = get_cache_db().await?;

//     // 1. 获取所有缓存数据
//     let all_cache_msg: Vec<AutoShutterCacheModel> = AutoShutterCacheEntity::find().all(db).await?;

//     // 2. 转换为 FullCacheData DTO 并返回
//     let result: Vec<FullCacheData> = all_cache_msg.into_iter().map(FullCacheData::from).collect();

//     Ok(result)
// }

// /// 查询缓存数据并返回 (供外部同步到持久化存储)
// /// TS: updateAllModelAutoShutterEntityCache (现改为返回参数)
// pub async fn get_all_model_auto_shutter_entity_cache_model_id(
//     model_id: String,
// ) -> Result<Vec<FullCacheData>, DbErr> {
//     let db = get_cache_db().await?;

//     // 1. 获取所有缓存数据
//     let all_cache_msg: Vec<AutoShutterCacheModel> = AutoShutterCacheEntity::find()
//         .filter(AutoShutterCacheColumn::ModelId.eq(model_id))
//         .all(db)
//         .await?;

//     // 2. 转换为 FullCacheData DTO 并返回
//     let result: Vec<FullCacheData> = all_cache_msg.into_iter().map(FullCacheData::from).collect();

//     Ok(result)
// }

// /// 获取快照列表 (返回 AutoShutterListItem)
// pub async fn get_all_model_auto_shutter_entity_list_cache(
//     order_flag: String, // "DESC" or "ASC"
//     auto_count: u64,
//     model_id: String,
// ) -> Result<Vec<AutoShutterListItem>, DbErr> {
//     let db = get_cache_db().await?;

//     let order = if order_flag.to_uppercase() == "ASC" {
//         Order::Asc
//     } else {
//         Order::Desc
//     };

//     let base_entity = AutoShutterCacheEntity::find()
//         .filter(AutoShutterCacheColumn::ModelId.eq(model_id))
//         .order_by(AutoShutterCacheColumn::UpdateAt, order)
//         .limit(auto_count)
//         .all(db)
//         .await?;
//     let result: Vec<AutoShutterListItem> = base_entity
//         .into_iter()
//         .map(AutoShutterListItem::from)
//         .collect();
//     Ok(result)
// }

// /// 获取单个详情 (返回 FullCacheData 或 DbErr)
// pub async fn get_model_auto_shutter_entity_by_id_cache(
//     id: i32,
//     model_id: String,
// ) -> Result<FullCacheData, DbErr> {
//     let db = get_cache_db().await?;
//     let model_id_clone = model_id.clone();
//     let result = AutoShutterCacheEntity::find_by_id(id)
//         .filter(AutoShutterCacheColumn::ModelId.eq(model_id))
//         .one(db)
//         .await?;

//     match result {
//         Some(model) => {
//             // 找到数据，进行转换并返回
//             Ok(FullCacheData::from(model)) // 假设 FullCacheData::from(model)
//         }
//         None => {
//             // 找不到数据，返回错误
//             Err(DbErr::RecordNotFound(format!(
//                 "ModelId: {} not found with ID: {}",
//                 model_id_clone, id
//             )))
//         }
//     }
// }
