use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use crate::service_database::database_physical_property::entity::physical_component_base_entity::{
    ActiveModel as PhysicalBaseActiveModel, Column as PhysicalBaseColumn,
    Entity as PhysicalBaseEntity, Model as PhysicalBaseModel,
};

use napi_derive::napi;
use sea_orm::Statement;
use sea_orm::{
    entity::prelude::*, ActiveValue::Set, ColumnTrait, EntityTrait, NotSet, QueryFilter,
    TransactionTrait,
};
use serde_json::Value;

use serde::{Deserialize, Serialize};

// 定义 DTO 供 NAPI 使用 (如果需要)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "physicalComponent", js_name = "PhysicalBaseDTO")]
pub struct PhysicalBaseDTO {
    pub id: i32,
    #[napi(js_name = "componentId")]
    pub component_id: i32,
    pub code: String,
    pub name: String,
    #[napi(js_name = "refId")]
    pub ref_id: i32,
    pub value: f64, // JS 数字对应 f64 更稳妥
    #[napi(js_name = "unitType")]
    pub unit_type: String,
    #[napi(js_name = "isDefault")]
    pub is_default: i32,
    #[napi(js_name = "isShow")]
    pub is_show: i32,
}

// 类型转换
impl From<PhysicalBaseModel> for PhysicalBaseDTO {
    fn from(ele: PhysicalBaseModel) -> Self {
        PhysicalBaseDTO {
            id: ele.id,
            name: ele.name,
            code: ele.code,
            component_id: ele.component_id,
            ref_id: ele.ref_id,
            value: ele.value,
            unit_type: ele.unit_type,
            is_default: ele.is_default,
            is_show: ele.is_show,
        }
    }
}

/// 根据 IDs 获取所有的记录
pub async fn get_physical_base_by_ids(ids: Vec<i32>) -> Result<Vec<PhysicalBaseDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let base_list: Vec<PhysicalBaseModel> = PhysicalBaseEntity::find()
        .filter(PhysicalBaseColumn::Id.is_in(ids))
        .all(db)
        .await?;
    let result = base_list.into_iter().map(PhysicalBaseDTO::from).collect();

    Ok(result)
}

/// 根据 compoundId 读取记录
pub async fn get_physical_base_by_compound_id(
    compound_id: i32,
) -> Result<Vec<PhysicalBaseDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let base_list = PhysicalBaseEntity::find()
        .filter(PhysicalBaseColumn::ComponentId.eq(compound_id))
        .all(db)
        .await?;
    let result = base_list.into_iter().map(PhysicalBaseDTO::from).collect();
    Ok(result)
}

pub async fn init_physical_base_data_fast(init_json_data: Vec<Value>) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    let txn = db.begin().await?;

    // 1. 清空表
    PhysicalBaseEntity::delete_many().exec(&txn).await?;

    // 2. 转换数据
    let mut max_id = 0;
    let models: Vec<PhysicalBaseActiveModel> = init_json_data
        .into_iter()
        .map(|item| {
            let id = item["id"].as_i64().unwrap_or(0) as i32;
            if id > max_id {
                max_id = id;
            } // 记录当前最大 ID

            PhysicalBaseActiveModel {
                id: item["id"].as_i64().map(|v| Set(v as i32)).unwrap_or(NotSet),
                component_id: Set(item["compoundId"].as_i64().unwrap_or(0) as i32),
                code: Set(item["code"].as_str().unwrap_or("").to_string()),
                name: Set(item["name"].as_str().unwrap_or("").to_string()),
                ref_id: Set(item["refId"].as_i64().unwrap_or(0) as i32),
                value: Set(item["value"].as_f64().unwrap_or(0.0)),
                unit_type: Set(item["unitType"].as_str().unwrap_or("").to_string()),
                is_default: Set(item["isDefault"].as_i64().unwrap_or(0) as i32),
                is_show: Set(item["isShow"].as_i64().unwrap_or(1) as i32),
            }
        })
        .collect();

    // 3. 批量分段插入
    for chunk in models.chunks(500) {
        PhysicalBaseEntity::insert_many(chunk.to_vec())
            .exec(&txn)
            .await?;
    }

    // 4. 重要：重置自增计数器，防止下次插入冲突
    // 根据你使用的数据库类型选择对应的 SQL
    let db_backend = txn.get_database_backend();
    let reset_sql: Option<String> = match db_backend {
        sea_orm::DatabaseBackend::MySql => Some(format!(
            "ALTER TABLE pp_component_base_entity AUTO_INCREMENT = {}",
            max_id + 1
        )),
        sea_orm::DatabaseBackend::Postgres => Some(format!(
            "SELECT setval(pg_get_serial_sequence('pp_component_base_entity', 'id'), {})",
            max_id
        )),
        sea_orm::DatabaseBackend::Sqlite => Some(format!(
            "UPDATE sqlite_sequence SET seq = {} WHERE name = 'pp_component_base_entity'",
            max_id
        )),
        _ => None,
    };

    if let Some(sql) = reset_sql {
        // 创建 Statement 对象
        let statement = Statement::from_string(db_backend, sql);
        // 传递 Statement 的引用给 execute()
        txn.execute_raw(statement).await?; // 传递引用 &statement
    } else {
        // Handle the case for unsupported database backends
        eprintln!("Warning: Unsupported database backend for resetting auto-increment.  ->pp_component_base_entity");
    }

    txn.commit().await?;
    Ok(())
}
