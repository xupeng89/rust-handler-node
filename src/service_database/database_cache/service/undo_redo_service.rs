use chrono::Utc;

use crate::service_database::database_cache::entity::model_undo_entity_cache::{
    self, ActiveModel, Column as UndoEntityColumn, Entity as UndoEntity, Model as UndoModel,
};

use sea_orm::{entity::*, query::*, ActiveValue::Set, DbErr, EntityTrait};
use serde::Deserialize;

// 导入公共数据库连接函数
use crate::service_database::database_cache::db_cache_connection::get_cache_db;
use napi_derive::napi;
// 对应传入的 TS 结构体
#[derive(Clone, Debug, Deserialize)]
#[napi(object, namespace = "undoRedoHandle")]
pub struct UndoRedoData {
    pub id: i32,
    pub op_type: String,
    pub table_name: String,
    pub old_data: String,
    pub new_data: String,
    pub model_id: String,
}

impl From<UndoModel> for UndoRedoData {
    fn from(model: UndoModel) -> Self {
        UndoRedoData {
            id: model.id,
            op_type: model.op_type,
            table_name: model.table_name,
            old_data: model.old_data,
            new_data: model.new_data, // 假设字段名匹配
            model_id: model.model_id,
        }
    }
}

/// 添加一条操作日志，返回插入ID
pub async fn add_undo_log_service(
    op_type: String,
    table_name: String,
    old_data: String,
    new_data: String,
    model_id: String,
) -> Result<i32, DbErr> {
    let db = get_cache_db().await?;
    // 删除 status = 1 或 2 的记录
    model_undo_entity_cache::Entity::delete_many()
        .filter(model_undo_entity_cache::Column::Status.is_in([1, 2]))
        .exec(db)
        .await?;

    let new_item = ActiveModel {
        model_id: Set(model_id),
        table_name: Set(table_name),
        op_type: Set(op_type),
        old_data: Set(old_data),
        new_data: Set(new_data),
        status: Set(0),
        operator_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let result = model_undo_entity_cache::Entity::insert(new_item)
        .exec(db)
        .await?;

    Ok(result.last_insert_id)
}

/// 查询某个模型的所有日志
pub async fn list_undo_logs_service(model_id: String) -> Result<Vec<UndoRedoData>, DbErr> {
    let db = get_cache_db().await?;
    let result = UndoEntity::find()
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .order_by_desc(UndoEntityColumn::Id)
        .all(db)
        .await?;

    let result_data: Vec<UndoRedoData> = result
        .into_iter()
        .map(UndoRedoData::from) // <-- 使用 From Trait
        .collect();

    Ok(result_data)
}

/// 更新日志状态
pub async fn update_status_service(id: i32, model_id: String, status: i32) -> Result<bool, DbErr> {
    let db = get_cache_db().await?;

    if let Some(record) = UndoEntity::find()
        .filter(UndoEntityColumn::Id.eq(id))
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .one(db)
        .await?
    {
        let mut active: ActiveModel = record.into();
        active.status = Set(status);
        active.update(db).await?;
        return Ok(true);
    }
    Ok(false)
}

/// 更新日志需要重制的信息
pub async fn update_new_data_service(
    id: i32,
    model_id: String,
    new_data: String,
) -> Result<bool, DbErr> {
    let db = get_cache_db().await?;

    if let Some(record) = UndoEntity::find()
        .filter(UndoEntityColumn::Id.eq(id))
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .one(db)
        .await?
    {
        let mut active: ActiveModel = record.into();
        active.new_data = Set(new_data);
        active.update(db).await?;
        return Ok(true);
    }
    Ok(false)
}

/// 删除某条日志
pub async fn delete_undo_log_service(id: i32, model_id: String) -> Result<bool, DbErr> {
    let db = get_cache_db().await?;
    let result = UndoEntity::delete_many()
        .filter(UndoEntityColumn::Id.eq(id))
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(result.rows_affected > 0)
}

/// 根据model_id删除日志记录
pub async fn delete_undo_log_service_by_model_id(model_id: String) -> Result<bool, DbErr> {
    let db = get_cache_db().await?;
    let result = UndoEntity::delete_many()
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(result.rows_affected > 0)
}

pub async fn get_latest_pending_log(model_id: String) -> Result<Option<UndoRedoData>, DbErr> {
    let db = get_cache_db().await?;

    let log = UndoEntity::find()
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .filter(UndoEntityColumn::Status.eq(0)) // status = 0
        .order_by_desc(UndoEntityColumn::OperatorAt) // 最近的时间在前
        .one(db)
        .await?;

    let result_data: Option<UndoRedoData> = log.map(UndoRedoData::from);

    Ok(result_data) // Option<Model>
}

pub async fn get_latest_redo_log(model_id: String) -> Result<Option<UndoRedoData>, DbErr> {
    let db = get_cache_db().await?;

    let log = UndoEntity::find()
        .filter(UndoEntityColumn::ModelId.eq(model_id))
        .filter(UndoEntityColumn::Status.eq(1)) // status = 1
        .order_by_desc(UndoEntityColumn::OperatorAt) // 最近的时间在前
        .one(db)
        .await?;
    let result_data: Option<UndoRedoData> = log.map(UndoRedoData::from);

    Ok(result_data) // Option<Model>
}

pub async fn bench_insert() -> Result<(), DbErr> {
    let db = get_cache_db().await?;

    let txn = db.begin().await?;

    for i in 0..100 {
        txn.execute_unprepared(&format!(
            "INSERT INTO model_undo_entity_cache
            (model_id, table_name, op_type, old_data, new_data, status, operator_at)
            VALUES ('model_{i}', 'user', 'INSERT', '{{}}', '{{\"id\":{i}}}', 0, '{}');",
            Utc::now().naive_utc()
        ))
        .await?;
    }

    txn.commit().await?;
    Ok(())
}
