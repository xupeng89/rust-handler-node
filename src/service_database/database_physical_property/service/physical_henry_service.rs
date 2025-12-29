use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, EntityTrait, QueryFilter,
    TransactionTrait, entity::prelude::*,
};

use napi_derive::napi;
use serde::{Deserialize, Serialize};
// 假设你的实体模块路径如下，请根据实际情况调整
use crate::service_database::database_physical_property::entity::physical_henry_detail_property_entity::{
    Entity as HenryEntity,
    Column as HenryColumn,
    ActiveModel as HenryActiveModel,
    Model as HenryModel
};

use serde_json::Value;

/// 同步亨利数据
pub async fn sync_pp_henry_detail_data(data: Vec<Value>) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    let txn = db.begin().await?;

    // 1. 提取传入数据中所有的 ID 用于后续做差集删除
    let incoming_ids: Vec<i32> = data
        .iter()
        .filter_map(|item| item["id"].as_i64().map(|v| v as i32))
        .collect();

    // 2. 差集删除：删除数据库里有，但传入列表里没有的数据
    HenryEntity::delete_many()
        .filter(HenryColumn::Id.is_not_in(incoming_ids))
        .exec(&txn)
        .await?;

    // 3. 循环处理每一条数据
    for item in data {
        if let Some(id) = item["id"].as_i64().map(|v| v as i32) {
            // 查询是否存在
            let existing = HenryEntity::find_by_id(id).one(&txn).await?;

            // 手动构建 ActiveModel，确保类型安全且不受 JSON 额外字段干扰
            let am = HenryActiveModel {
                id: Set(id),
                component_i_id: Set(item["soluteId"].as_i64().unwrap_or(0) as i32),
                component_i: Set(item["solute"].as_str().unwrap_or("").to_string()),
                component_j_id: Set(item["solventId"].as_i64().unwrap_or(0) as i32),
                component_j: Set(item["solvent"].as_str().unwrap_or("").to_string()),
                source_name: Set(item["sourceName"].as_str().unwrap_or("").to_string()),
                aij: Set(item["AIJ"].as_f64().unwrap_or(0.0)),
                bij: Set(item["BIJ"].as_f64().unwrap_or(0.0)),
                cij: Set(item["CIJ"].as_f64().unwrap_or(0.0)),
                dij: Set(item["DIJ"].as_f64().unwrap_or(0.0)),
                eij: Set(item["EIJ"].as_f64().unwrap_or(0.0)),
                t_lower: Set(item["TLOWER"].as_f64().unwrap_or(0.0)),
                t_upper: Set(item["TUPPER"].as_f64().unwrap_or(0.0)),
            };

            if existing.is_some() {
                // 如果存在，执行更新
                am.update(&txn).await?;
            } else {
                // 如果不存在，执行插入
                am.insert(&txn).await?;
            }
        }
    }

    txn.commit().await?;
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[napi(object, namespace = "heryDetail", js_name = "HenryDetailDTO")]
pub struct HenryDetailDTO {
    pub id: i32,
    #[napi(js_name = "componentI")]
    pub component_i: String,
    #[napi(js_name = "componentJ")]
    pub component_j: String,
    #[napi(js_name = "sourceName")]
    pub source_name: String,
    #[napi(js_name = "AIJ")]
    pub aij: f64,
    #[napi(js_name = "BIJ")]
    pub bij: f64,
    #[napi(js_name = "CIJ")]
    pub cij: f64,
    #[napi(js_name = "DIJ")]
    pub dij: f64,
    #[napi(js_name = "EIJ")]
    pub eij: f64,
    #[napi(js_name = "TLOWER")]
    pub t_lower: f64,
    #[napi(js_name = "TUPPER")]
    pub t_upper: f64,
    #[napi(js_name = "componentIId")]
    pub component_i_id: i32,
    #[napi(js_name = "componentJId")]
    pub component_j_id: i32,
}

// 实现从数据库 Model 转换到 DTO
impl From<HenryModel> for HenryDetailDTO {
    fn from(model: HenryModel) -> Self {
        Self {
            id: model.id,
            component_i: model.component_i,
            component_j: model.component_j,
            source_name: model.source_name,
            aij: model.aij,
            bij: model.bij,
            cij: model.cij,
            dij: model.dij,
            eij: model.eij,
            t_lower: model.t_lower,
            t_upper: model.t_upper,
            component_i_id: model.component_i_id,
            component_j_id: model.component_j_id,
        }
    }
}
/// 2. 根据组分 I 列表和 J 列表查询多条信息 (IN 查询)
pub async fn query_pp_component_henry_detail_data_by_i_or_j(
    ids_i: Vec<String>,
    ids_j: Vec<String>,
) -> Result<Vec<HenryDetailDTO>, DbErr> {
    let db = get_physical_property_db().await?;

    let henry_list = HenryEntity::find()
        .filter(
            Condition::all()
                .add(HenryColumn::ComponentI.is_in(ids_i))
                .add(HenryColumn::ComponentJ.is_in(ids_j)),
        )
        .all(db)
        .await?;

    let result = henry_list.into_iter().map(HenryDetailDTO::from).collect();

    Ok(result)
}
