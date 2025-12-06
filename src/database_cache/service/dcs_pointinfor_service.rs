use crate::database_cache::db_cache_connection::get_cache_db;
use napi_derive::napi;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set,
    TransactionError, TransactionTrait,
};
use serde::Deserialize;
use std::collections::HashSet;
// 导入点位信息实体
use crate::database_cache::entity::model_pointinfor_entity_cache::{
    ActiveModel, Column as PointColumn, Entity as PointInforEntity, Model as PointInforModel,
};

// 对应传入的 TS 结构体
#[derive(Clone, Debug, Deserialize)]
#[napi(object)]
pub struct PositionData {
    pub name: String,
    pub name_display: String,
    pub type_name: String,
    // 注意：Rust 命名约定使用 snake_case (type_num)，但为了兼容可能用 type
    #[serde(rename = "type")] // 假设外部 JSON 仍使用 "type" 字段名
    pub type_num: i32,
}

impl From<PointInforModel> for PositionData {
    fn from(model: PointInforModel) -> Self {
        PositionData {
            name: model.name,
            name_display: model.name_display,
            type_name: model.type_name,
            type_num: model.type_num, // 假设字段名匹配
        }
    }
}

// ======================================
// 1. 根据类型批量查询 (getPositionInformationCacheByTypes)
// ======================================
/// 根据 type_num 列表查询点位信息
pub async fn get_position_information_cache_by_types(
    types: Vec<i32>, // Rust 中数组使用 Vec
) -> Result<Vec<PositionData>, DbErr> {
    let db = get_cache_db().await?;

    // SeaORM 使用 is_in 方法实现 SQL 的 IN 子句
    let result = PointInforEntity::find()
        .filter(PointColumn::TypeNum.is_in(types))
        .all(db)
        .await?;

    let result_data: Vec<PositionData> = result
        .into_iter()
        .map(PositionData::from) // <-- 使用 From Trait
        .collect();

    Ok(result_data)
}

// ======================================
// 2. 查询所有信息 (getPositionInformationCacheAllMessage)
// ======================================
/// 查询所有点位信息
pub async fn get_position_information_cache_all_message() -> Result<Vec<PositionData>, DbErr> {
    let db = get_cache_db().await?;

    let result = PointInforEntity::find().all(db).await?;

    let result_data: Vec<PositionData> = result
        .into_iter()
        .map(PositionData::from) // <-- 使用 From Trait
        .collect();

    Ok(result_data)
}

/// 批量更新或插入数据 (包含删除逻辑)
pub async fn update_or_insert_position_information_cache(
    data: Vec<PositionData>,
) -> Result<(), DbErr> {
    if data.is_empty() {
        return Ok(());
    }
    let db = get_cache_db().await?;

    // ----------------------------------------------------
    // 修复点：使用 ::<_, (), DbErr> 显式指定类型
    // <_, (), DbErr> 含义：
    //   _ : 让编译器推断闭包类型
    //   () : 事务成功返回的类型 (Ok(()))
    //   DbErr : 事务失败返回的错误类型
    // ----------------------------------------------------
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            // Step 1: 获取当前数据库中所有 name 和对应的 Model
            let existing_records = PointInforEntity::find().all(txn).await?;

            let existing_map: std::collections::HashMap<String, ActiveModel> = existing_records
                .into_iter()
                .map(|m| (m.name.clone(), m.into_active_model()))
                .collect();

            let incoming_names_set: HashSet<String> =
                data.iter().map(|item| item.name.clone()).collect();
            let existing_names_set: HashSet<String> = existing_map.keys().cloned().collect();

            // Step 2: 区分待处理列表
            let mut to_insert: Vec<ActiveModel> = Vec::new();
            let mut to_update_models: Vec<ActiveModel> = Vec::new();

            for item in data {
                if let Some(existing_record) = existing_map.get(&item.name) {
                    // 更新
                    let mut active_model: ActiveModel = existing_record.clone();
                    active_model.name_display = Set(item.name_display);
                    active_model.type_num = Set(item.type_num);
                    active_model.type_name = Set(item.type_name);
                    to_update_models.push(active_model);
                } else {
                    // 插入
                    to_insert.push(ActiveModel {
                        name: Set(item.name),
                        name_display: Set(item.name_display),
                        type_num: Set(item.type_num),
                        type_name: Set(item.type_name),
                        ..Default::default()
                    });
                }
            }

            // Step 3: 执行操作
            let to_delete: Vec<String> = existing_names_set
                .difference(&incoming_names_set)
                .cloned()
                .collect();

            // 1. 删除
            if !to_delete.is_empty() {
                PointInforEntity::delete_many()
                    .filter(PointColumn::Name.is_in(to_delete))
                    .exec(txn)
                    .await?;
            }

            // 2. 更新
            for active in to_update_models {
                active.update(txn).await?;
            }

            // 3. 插入
            if !to_insert.is_empty() {
                PointInforEntity::insert_many(to_insert).exec(txn).await?;
            }

            Ok(())
        })
    })
    .await
    // 手动处理 TransactionError，将其转换回 DbErr
    .map_err(|e| match e {
        TransactionError::Connection(db_err) => db_err,
        TransactionError::Transaction(db_err) => db_err,
    })?;

    Ok(())
}
