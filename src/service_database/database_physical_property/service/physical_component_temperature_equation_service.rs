use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use crate::service_database::database_physical_property::entity::physical_component_temperature_equation_entity::{
    ActiveModel as TemperatureEquationActiveModel, Column as TemperatureEquationColumn,
    Entity as TemperatureEquationEntity, Model as TemperatureEquationModel,
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
#[napi(object, namespace = "physicalComponent")]
pub struct TemperatureEquationDTO {
    pub id: i32,
    #[napi(js_name = "componentId")]
    pub component_id: i32,
    pub code: String,
    pub name: String,
    pub unit: String,
    #[napi(js_name = "functionCode")]
    pub function_code: i32,
    #[napi(js_name = "functionName")]
    pub function_name: String,
    #[napi(js_name = "coefficientA")]
    pub coefficient_a: f64,
    #[napi(js_name = "coefficientB")]
    pub coefficient_b: f64,
    #[napi(js_name = "coefficientC")]
    pub coefficient_c: f64,
    #[napi(js_name = "coefficientD")]
    pub coefficient_d: f64,
    #[napi(js_name = "coefficientE")]
    pub coefficient_e: f64,
    #[napi(js_name = "coefficientF")]
    pub coefficient_f: f64,
    #[napi(js_name = "coefficientG")]
    pub coefficient_g: f64,
    #[napi(js_name = "coefficientH")]
    pub coefficient_h: f64,
    #[napi(js_name = "coefficientK")]
    pub coefficient_k: f64,
    #[napi(js_name = "coefficientL")]
    pub coefficient_l: f64,
    #[napi(js_name = "minApplicableTemperature")]
    pub min_applicable_temperature: f64,
    #[napi(js_name = "minApplicableTemperatureUnit")]
    pub min_applicable_temperature_unit: String,
    #[napi(js_name = "maxApplicableTemperature")]
    pub max_applicable_temperature: f64,
    #[napi(js_name = "maxApplicableTemperatureUnit")]
    pub max_applicable_temperature_unit: String,
    #[napi(js_name = "isDefault")]
    pub is_default: i32,
    #[napi(js_name = "isShow")]
    pub is_show: i32,
}

// 类型转换
impl From<TemperatureEquationModel> for TemperatureEquationDTO {
    fn from(ele: TemperatureEquationModel) -> Self {
        TemperatureEquationDTO {
            id: ele.id,
            component_id: ele.component_id,
            name: ele.name,
            code: ele.code,
            unit: ele.unit,
            function_code: ele.function_code,
            function_name: ele.function_name,
            coefficient_a: ele.coefficient_a,
            coefficient_b: ele.coefficient_b,
            coefficient_c: ele.coefficient_c,
            coefficient_d: ele.coefficient_d,
            coefficient_e: ele.coefficient_e,
            coefficient_f: ele.coefficient_f,
            coefficient_g: ele.coefficient_g,
            coefficient_h: ele.coefficient_h,
            coefficient_k: ele.coefficient_k,
            coefficient_l: ele.coefficient_l,
            min_applicable_temperature: ele.min_applicable_temperature,
            min_applicable_temperature_unit: ele.min_applicable_temperature_unit,
            max_applicable_temperature: ele.max_applicable_temperature,
            max_applicable_temperature_unit: ele.max_applicable_temperature_unit,
            is_default: ele.is_default,
            is_show: ele.is_show,
        }
    }
}
/// 根据 IDs 获取所有的记录
pub async fn get_physical_temperature_equation_by_ids(
    ids: Vec<i32>,
) -> Result<Vec<TemperatureEquationDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let temp_list = TemperatureEquationEntity::find()
        .filter(TemperatureEquationColumn::Id.is_in(ids))
        .all(db)
        .await?;

    let result = temp_list
        .into_iter()
        .map(TemperatureEquationDTO::from)
        .collect();

    Ok(result)
}

/// 根据 compoundId 读取记录
pub async fn get_physical_temperature_equation_by_compound_id(
    compound_id: i32,
) -> Result<Vec<TemperatureEquationDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let temp_list = TemperatureEquationEntity::find()
        .filter(TemperatureEquationColumn::ComponentId.eq(compound_id))
        .all(db)
        .await?;

    let result = temp_list
        .into_iter()
        .map(TemperatureEquationDTO::from)
        .collect();

    Ok(result)
}

pub async fn init_physical_temperature_equation_data_fast(
    init_json_data: Vec<Value>,
) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    let txn = db.begin().await?;

    // 1. 清空表
    TemperatureEquationEntity::delete_many().exec(&txn).await?;

    // 2. 转换数据
    let mut max_id = 0;
    let models: Vec<TemperatureEquationActiveModel> = init_json_data
        .into_iter()
        .map(|item| {
            let id = item["id"].as_i64().unwrap_or(0) as i32;
            if id > max_id {
                max_id = id;
            } // 记录当前最大 ID

            TemperatureEquationActiveModel {
                id: item["id"].as_i64().map(|v| Set(v as i32)).unwrap_or(NotSet),
                component_id: Set(item["componentId"].as_i64().unwrap_or(0) as i32),
                code: Set(item["code"].as_str().unwrap_or("").to_string()),
                name: Set(item["name"].as_str().unwrap_or("").to_string()),
                unit: Set(item["unit"].as_str().unwrap_or("").to_string()),
                function_code: Set(item["functionCode"].as_i64().unwrap_or(0) as i32),
                function_name: Set(item["functionName"].as_str().unwrap_or("").to_string()),
                coefficient_a: Set(item["coefficientA"].as_f64().unwrap_or(0.0)),
                coefficient_b: Set(item["coefficientB"].as_f64().unwrap_or(0.0)),
                coefficient_c: Set(item["coefficientC"].as_f64().unwrap_or(0.0)),
                coefficient_d: Set(item["coefficientD"].as_f64().unwrap_or(0.0)),
                coefficient_e: Set(item["coefficientE"].as_f64().unwrap_or(0.0)),
                coefficient_f: Set(item["coefficientF"].as_f64().unwrap_or(0.0)),
                coefficient_g: Set(item["coefficientG"].as_f64().unwrap_or(0.0)),
                coefficient_h: Set(item["coefficientH"].as_f64().unwrap_or(0.0)),
                coefficient_k: Set(item["coefficientK"].as_f64().unwrap_or(0.0)),
                coefficient_l: Set(item["coefficientL"].as_f64().unwrap_or(0.0)),
                min_applicable_temperature: Set(item["minApplicableTemperature"]
                    .as_f64()
                    .unwrap_or(0.0)),
                min_applicable_temperature_unit: Set(item["minApplicableTemperatureUnit"]
                    .as_str()
                    .unwrap_or("")
                    .to_string()),
                max_applicable_temperature: Set(item["maxApplicableTemperature"]
                    .as_f64()
                    .unwrap_or(0.0)),
                max_applicable_temperature_unit: Set(item["maxApplicableTemperatureUnit"]
                    .as_str()
                    .unwrap_or("")
                    .to_string()),

                is_default: Set(item["isDefault"].as_i64().unwrap_or(0) as i32),
                is_show: Set(item["isShow"].as_i64().unwrap_or(1) as i32),
            }
        })
        .collect();

    // 3. 批量分段插入
    for chunk in models.chunks(500) {
        TemperatureEquationEntity::insert_many(chunk.to_vec())
            .exec(&txn)
            .await?;
    }

    // 4. 重要：重置自增计数器，防止下次插入冲突
    // 根据你使用的数据库类型选择对应的 SQL
    let db_backend = txn.get_database_backend();
    let reset_sql: Option<String> = match db_backend {
        sea_orm::DatabaseBackend::MySql => Some(format!(
            "ALTER TABLE pp_component_temperature_equation_entity AUTO_INCREMENT = {}",
            max_id + 1
        )),
        sea_orm::DatabaseBackend::Postgres => Some(format!(
            "SELECT setval(pg_get_serial_sequence('pp_component_temperature_equation_entity', 'id'), {})",
            max_id
        )),
        sea_orm::DatabaseBackend::Sqlite => Some(format!(
            "UPDATE sqlite_sequence SET seq = {} WHERE name = 'pp_component_temperature_equation_entity'",
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
        eprintln!("Warning: Unsupported database backend for resetting auto-increment.   ->pp_component_temperature_equation_entity");
    }

    txn.commit().await?;
    Ok(())
}
