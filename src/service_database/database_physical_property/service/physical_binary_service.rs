use crate::service_database::{
    database_physical_property::db_physical_property_connection::get_physical_property_db,
    interface_trait::SyncableBinaryActiveModel,
    interface_trait::{HasId, SyncableBinaryEntity},
};
use sea_orm::{
    ExprTrait, IntoActiveModel, TransactionTrait, TryIntoModel,
    entity::prelude::*,
    sea_query::{Alias, Expr},
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
    pub async fn generic_sync<E, AM, F>(
        db: &DatabaseConnection,
        incoming_data: Vec<Value>,
        map_fn: F,
    ) -> Result<(), DbErr>
    where
        E: SyncableBinaryEntity,
        AM: ActiveModelTrait<Entity = E>
            + ActiveModelBehavior
            + SyncableBinaryActiveModel
            + Send
            + TryIntoModel<E::Model>,
        // 关键修复：添加了 TryIntoModel 约束，以便从 ActiveModel 转回 Model
        E::Model: IntoActiveModel<AM> + HasId + Serialize + for<'de> Deserialize<'de> + Sync,
        F: Fn(&mut AM, &Value),
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
                .filter(Expr::col(Alias::new(E::col_i())).eq(i_id))
                .filter(Expr::col(Alias::new(E::col_j())).eq(j_id))
                .one(&txn)
                .await?;

            if let Some(model) = existing {
                processed_ids.push(model.get_id());

                let mut am = model.into_active_model();
                SyncableBinaryActiveModel::sync_set_from_json(&mut am, item_json.clone())?;
                map_fn(&mut am, &item_json);
                am.update(&txn).await?;
            } else {
                let mut am = AM::default();

                // am.sync_set_from_json(item_json)?;
                SyncableBinaryActiveModel::sync_set_from_json(&mut am, item_json.clone())?;
                map_fn(&mut am, &item_json);
                let inserted_model = am.insert(&txn).await?;

                // 【优化】插入后也直接获取 ID
                processed_ids.push(inserted_model.get_id());
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
        "PR" => {
            BinarySyncService::generic_sync::<pr::Entity, pr::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    // 这里写 PR 特有的映射
                    if let Some(v) = json["KAIJ"].as_str() {
                        am.kaij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KBIJ"].as_str() {
                        am.kbij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KCIJ"].as_str() {
                        am.kcij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "RK" => {
            BinarySyncService::generic_sync::<rk::Entity, rk::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    // 这里写 PR 特有的映射
                    if let Some(v) = json["KAIJ"].as_str() {
                        am.kaij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KBIJ"].as_str() {
                        am.kbij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KCIJ"].as_str() {
                        am.kcij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "SRK" => {
            BinarySyncService::generic_sync::<srk::Entity, srk::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    // 这里写 PR 特有的映射
                    if let Some(v) = json["KAIJ"].as_str() {
                        am.kaij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KBIJ"].as_str() {
                        am.kbij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["KCIJ"].as_str() {
                        am.kcij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "NRTL" => {
            BinarySyncService::generic_sync::<nrtl::Entity, nrtl::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    if let Some(v) = json["AIJ"].as_str() {
                        am.aij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["AJI"].as_str() {
                        am.aji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BIJ"].as_str() {
                        am.bij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BJI"].as_str() {
                        am.bji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["CIJ"].as_str() {
                        am.cij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["DIJ"].as_str() {
                        am.dij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EIJ"].as_str() {
                        am.eij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EJI"].as_str() {
                        am.eji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FIJ"].as_str() {
                        am.fij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FJI"].as_str() {
                        am.fji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "NRTLRK" => {
            BinarySyncService::generic_sync::<nrtl_rk::Entity, nrtl_rk::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    if let Some(v) = json["AIJ"].as_str() {
                        am.aij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["AJI"].as_str() {
                        am.aji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BIJ"].as_str() {
                        am.bij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BJI"].as_str() {
                        am.bji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["CIJ"].as_str() {
                        am.cij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["DIJ"].as_str() {
                        am.dij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EIJ"].as_str() {
                        am.eij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EJI"].as_str() {
                        am.eji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FIJ"].as_str() {
                        am.fij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FJI"].as_str() {
                        am.fji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "PSRK" => {
            BinarySyncService::generic_sync::<psrk::Entity, psrk::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    // 这里写 PR 特有的映射
                    if let Some(v) = json["TIJ"].as_str() {
                        am.tij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["TJI"].as_str() {
                        am.tji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["VIJ"].as_str() {
                        am.vij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["VJI"].as_str() {
                        am.vji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "UNIQUAC" => {
            BinarySyncService::generic_sync::<uniquac::Entity, uniquac::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    if let Some(v) = json["AIJ"].as_str() {
                        am.aij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["AJI"].as_str() {
                        am.aji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BIJ"].as_str() {
                        am.bij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BJI"].as_str() {
                        am.bji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["CIJ"].as_str() {
                        am.cij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["DIJ"].as_str() {
                        am.dij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EIJ"].as_str() {
                        am.eij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EJI"].as_str() {
                        am.eji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FIJ"].as_str() {
                        am.fij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FJI"].as_str() {
                        am.fji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
        }
        "WILSON" => {
            BinarySyncService::generic_sync::<wilson::Entity, wilson::ActiveModel, _>(
                db,
                data,
                |am, json| {
                    if let Some(v) = json["AIJ"].as_str() {
                        am.aij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["AJI"].as_str() {
                        am.aji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BIJ"].as_str() {
                        am.bij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["BJI"].as_str() {
                        am.bji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["CIJ"].as_str() {
                        am.cij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["DIJ"].as_str() {
                        am.dij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EIJ"].as_str() {
                        am.eij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["EJI"].as_str() {
                        am.eji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FIJ"].as_str() {
                        am.fij = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["FJI"].as_str() {
                        am.fji = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["minT"].as_str() {
                        am.min_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                    if let Some(v) = json["maxT"].as_str() {
                        am.max_t = sea_orm::ActiveValue::Set(v.to_string());
                    }
                },
            )
            .await
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
        "PR" => BinarySyncService::generic_query::<pr::Entity>(db, ids).await,
        "RK" => BinarySyncService::generic_query::<rk::Entity>(db, ids).await,
        "SRK" => BinarySyncService::generic_query::<srk::Entity>(db, ids).await,
        "NRTL" => BinarySyncService::generic_query::<nrtl::Entity>(db, ids).await,
        "NRTLRK" => BinarySyncService::generic_query::<nrtl_rk::Entity>(db, ids).await,
        "PSRK" => BinarySyncService::generic_query::<psrk::Entity>(db, ids).await,
        "UNIQUAC" => BinarySyncService::generic_query::<uniquac::Entity>(db, ids).await,
        "WILSON" => BinarySyncService::generic_query::<wilson::Entity>(db, ids).await,
        _ => Ok(vec![]),
    }
}
