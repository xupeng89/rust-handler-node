use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use sea_orm::{
    entity::prelude::*,
    sea_query::{Alias, Expr},
    ExprTrait, IntoActiveModel, TransactionTrait, TryIntoModel,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::service_database::database_physical_property::entity::{
    physical_binary_nrtl_entity as nrtl, physical_binary_nrtl_rk_entity as nrtl_rk,
    physical_binary_pr_entity as pr, physical_binary_psrk_entity as psrk,
    physical_binary_rk_entity as rk, physical_binary_srk_entity as srk,
    physical_binary_uniquac_entity as uniquac, physical_binary_wilsion_entity as wilson,
};

struct BinarySyncService;

impl BinarySyncService {
    // ======================================
    // 1. 通用同步逻辑 (Insert/Update/Delete)
    // ======================================
    pub async fn generic_sync<E, AM>(
        db: &DatabaseConnection,
        incoming_data: Vec<Value>,
    ) -> Result<(), DbErr>
    where
        E: EntityTrait,
        AM: ActiveModelTrait<Entity = E> + ActiveModelBehavior + Send,
        // 关键修复：添加了 TryIntoModel 约束，以便从 ActiveModel 转回 Model
        AM: TryIntoModel<E::Model>,
        E::Model: IntoActiveModel<AM> + Serialize + for<'de> Deserialize<'de> + Sync,
    {
        let txn = db.begin().await?;
        let mut processed_ids = Vec::new();

        for item_json in incoming_data {
            // 【修复 BUG】: 使用 .map 转换类型，避免 () as i32 错误
            let i_id = item_json["componentIId"]
                .as_i64()
                .map(|v| v as i32)
                .ok_or_else(|| DbErr::Custom("Missing componentIId".into()))?;

            let j_id = item_json["componentJId"]
                .as_i64()
                .map(|v| v as i32)
                .ok_or_else(|| DbErr::Custom("Missing componentJId".into()))?;

            let existing = E::find()
                .filter(Expr::col(Alias::new("component_i_id")).eq(i_id))
                .filter(Expr::col(Alias::new("component_j_id")).eq(j_id))
                .one(&txn)
                .await?;

            if let Some(model) = existing {
                // 1. 记录已有 ID (利用序列化避开 PrimaryKey::Id 类型无法匹配的问题)
                let model_val =
                    serde_json::to_value(&model).map_err(|e| DbErr::Custom(e.to_string()))?;
                if let Some(id) = model_val["id"].as_i64() {
                    processed_ids.push(id as i32);
                }

                // 2. 更新
                let mut am = model.into_active_model();
                am.set_from_json(item_json)?;
                am.update(&txn).await?;
            } else {
                // 3. 插入
                let mut am = AM::default();
                am.set_from_json(item_json)?;
                let inserted_model = am.insert(&txn).await?;

                // 记录新插入生成的 ID
                let inserted_val = serde_json::to_value(&inserted_model)
                    .map_err(|e| DbErr::Custom(e.to_string()))?;
                if let Some(id) = inserted_val["id"].as_i64() {
                    processed_ids.push(id as i32);
                }
            }
        }

        // 4. 删除：清理旧数据
        if !processed_ids.is_empty() {
            E::delete_many()
                .filter(Expr::col(Alias::new("id")).is_not_in(processed_ids))
                .exec(&txn)
                .await?;
        } else {
            E::delete_many().exec(&txn).await?;
        }

        txn.commit().await?;
        Ok(())
    }

    // ======================================
    // 2. 通用查询逻辑 (对应 TS 的 queryByIorJList)
    // ======================================
    pub async fn generic_query<E>(
        db: &DatabaseConnection,
        ids: Vec<String>,
    ) -> Result<Vec<Value>, DbErr>
    where
        E: EntityTrait,
        E::Model: Serialize,
    {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        // 对应 TS 逻辑：componentI 在 ids 中 且 compound_j 在 ids 中
        let models = E::find()
            .filter(Expr::col(Alias::new("component_i")).is_in(ids.clone()))
            .filter(Expr::col(Alias::new("component_j")).is_in(ids))
            .all(db)
            .await?;

        // 转换为 JSON Value 以便 NAPI 返回给前端
        let result = models
            .into_iter()
            .map(|m| serde_json::to_value(m).unwrap())
            .collect();

        Ok(result)
    }
}

// ======================================
// 3. 统一分发入口
// ======================================

/// 批量同步分发
pub async fn dispatch_sync_request(func_code: &str, data: Vec<Value>) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    match func_code {
        "PR" => BinarySyncService::generic_sync::<pr::Entity, pr::ActiveModel>(&db, data).await,
        "RK" => BinarySyncService::generic_sync::<rk::Entity, rk::ActiveModel>(&db, data).await,
        "SRK" => BinarySyncService::generic_sync::<srk::Entity, srk::ActiveModel>(&db, data).await,
        "NRTL" => {
            BinarySyncService::generic_sync::<nrtl::Entity, nrtl::ActiveModel>(&db, data).await
        }
        "NRTL-RK" => {
            BinarySyncService::generic_sync::<nrtl_rk::Entity, nrtl_rk::ActiveModel>(&db, data)
                .await
        }
        "PSRK" => {
            BinarySyncService::generic_sync::<psrk::Entity, psrk::ActiveModel>(&db, data).await
        }
        "UNIQUAC" => {
            BinarySyncService::generic_sync::<uniquac::Entity, uniquac::ActiveModel>(&db, data)
                .await
        }
        "WILSON" => {
            BinarySyncService::generic_sync::<wilson::Entity, wilson::ActiveModel>(&db, data).await
        }
        _ => Err(DbErr::Custom(format!("不支持的方程编码: {}", func_code))),
    }
}

/// 批量查询分发 (对应你提供的 TS 查询接口)
pub async fn dispatch_query_request(
    func_code: &str,
    ids: Vec<String>,
) -> Result<Vec<Value>, DbErr> {
    let db = get_physical_property_db().await?;
    match func_code {
        "PR" => BinarySyncService::generic_query::<pr::Entity>(&db, ids).await,
        "RK" => BinarySyncService::generic_query::<rk::Entity>(&db, ids).await,
        "SRK" => BinarySyncService::generic_query::<srk::Entity>(&db, ids).await,
        "NRTL" => BinarySyncService::generic_query::<nrtl::Entity>(&db, ids).await,
        "NRTL-RK" => BinarySyncService::generic_query::<nrtl_rk::Entity>(&db, ids).await,
        "PSRK" => BinarySyncService::generic_query::<psrk::Entity>(&db, ids).await,
        "UNIQUAC" => BinarySyncService::generic_query::<uniquac::Entity>(&db, ids).await,
        "WILSON" => BinarySyncService::generic_query::<wilson::Entity>(&db, ids).await,
        _ => Ok(vec![]),
    }
}
