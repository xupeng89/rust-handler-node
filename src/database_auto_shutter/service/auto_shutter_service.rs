use crate::database_auto_shutter::db_auto_shutter_connection::get_auto_shutter_db;

use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Set, Statement};

// ======================================
// 引入实体定义
// ======================================
// 1. 缓存表实体
use crate::database_auto_shutter::entity::model_auto_shutter_entity::{
    ActiveModel as AutoShutterActiveModel, Entity as AutoShutterEntity, Model as AutoShutterModel,
};
use crate::database_cache::service::auto_shutter_cache_service::FullCacheData;

// ======================================
// DTO 定义 (已调整，新增 FullCacheData 用于数据同步)
// ======================================

// 读取完整数据
impl From<AutoShutterModel> for FullCacheData {
    fn from(ele: AutoShutterModel) -> Self {
        FullCacheData {
            id: ele.id,
            model_id: ele.model_id,
            objects: ele.objects,
            sysvars: ele.sysvars,
            update_at: ele.update_at,
            sim_time: ele.sim_time,
            status: ele.status,
            user_name: None,
            state_index: None,
            state_desc: None,
        }
    }
}

/// 批量插入数据到缓存数据库
/// TS: readModelAutoShutterEntityCache (现改为接受数据作为参数)
pub async fn read_current_model_auto_shutter_entity(data: Vec<FullCacheData>) -> Result<(), DbErr> {
    let db = get_auto_shutter_db().await?;

    AutoShutterEntity::delete_many().exec(db).await?;
    let backend = db.get_database_backend();
    let reset_sql = "DELETE FROM sqlite_sequence WHERE name = 'model_auto_shutter_entity';";
    db.execute_raw(Statement::from_string(backend, reset_sql.to_string()))
        .await?;
    // 转换为 ActiveModel
    let cache_inserts: Vec<AutoShutterActiveModel> = data
        .into_iter()
        .map(|d| AutoShutterActiveModel {
            id: Set(d.id),
            model_id: Set(d.model_id),
            objects: Set(d.objects),
            sysvars: Set(d.sysvars),
            update_at: Set(d.update_at),
            sim_time: Set(d.sim_time),
            status: Set(d.status),
            ..Default::default()
        })
        .collect();

    // 批量插入缓存表
    if !cache_inserts.is_empty() {
        AutoShutterEntity::insert_many(cache_inserts)
            .exec(db)
            .await?;
    }

    Ok(())
}

/// 查询缓存数据并返回 (供外部同步到持久化存储)
/// TS: updateAllModelAutoShutterEntityCache (现改为返回参数)
pub async fn get_current_all_model_auto_shutter_entity() -> Result<Vec<FullCacheData>, DbErr> {
    let db = get_auto_shutter_db().await?;

    // 1. 获取所有缓存数据
    let all_cache_msg: Vec<AutoShutterModel> = AutoShutterEntity::find().all(db).await?;

    // 2. 转换为 FullCacheData DTO 并返回
    let result: Vec<FullCacheData> = all_cache_msg.into_iter().map(FullCacheData::from).collect();

    Ok(result)
}
