use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use crate::service_database::database_physical_property::entity::physical_component_information_entity::{
    ActiveModel as InformationActiveModel, Column as InformationColumn,
    Entity as InformationEntity, Model as InformationModel,
};
use napi_derive::napi;
use sea_orm::Statement;
use sea_orm::{
    entity::prelude::*, ActiveValue::Set, ColumnTrait, EntityTrait, NotSet, QueryFilter,
    TransactionTrait,
};
use serde_json::Value;
// 定义 DTO 供 NAPI 使用 (如果需要)
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[napi(object, namespace = "physicalComponent")]
pub struct PhysicalInformationDTO {
    pub id: i32,
    pub name: String,
    pub formula: String,
    #[napi(js_name = "casNo")]
    pub cas_no: String,
    pub number: i32,
    #[napi(js_name = "internalName")]
    pub internal_name: String,
}

// 类型转换
impl From<InformationModel> for PhysicalInformationDTO {
    fn from(ele: InformationModel) -> Self {
        PhysicalInformationDTO {
            id: ele.id,
            name: ele.name,
            formula: ele.formula,
            cas_no: ele.cas_no,
            number: ele.number,
            internal_name: ele.internal_name,
        }
    }
}
/// 根据 casNo 获取单条记录，理论上casNo是唯一的
pub async fn get_physical_information_one_by_cas_no(
    cas_no: String,
) -> Result<PhysicalInformationDTO, DbErr> {
    let db = get_physical_property_db().await?;
    let model = InformationEntity::find()
        .filter(InformationColumn::CasNo.eq(cas_no))
        .one(db)
        .await?;

    let dto = model.map(PhysicalInformationDTO::from).unwrap();
    Ok(dto)
}

pub async fn get_physical_information_list_by_cas_no_list(
    cas_no_list: Vec<String>,
) -> Result<Vec<PhysicalInformationDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let model_list = InformationEntity::find()
        .filter(InformationColumn::CasNo.is_in(cas_no_list))
        .all(db)
        .await?;
    let result = model_list
        .into_iter()
        .map(PhysicalInformationDTO::from)
        .collect();
    Ok(result)
}

/// 获取所有记录
pub async fn get_all_physical_information_list() -> Result<Vec<PhysicalInformationDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let model_list = InformationEntity::find().all(db).await?;
    let result = model_list
        .into_iter()
        .map(PhysicalInformationDTO::from)
        .collect();
    Ok(result)
}

pub async fn init_physical_information_data_fast(init_json_data: Vec<Value>) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    let txn = db.begin().await?;

    // 1. 清空表
    InformationEntity::delete_many().exec(&txn).await?;

    // 2. 转换数据
    let mut max_id = 0;
    let models: Vec<InformationActiveModel> = init_json_data
        .into_iter()
        .map(|item| {
            let id = item["id"].as_i64().unwrap_or(0) as i32;
            if id > max_id {
                max_id = id;
            } // 记录当前最大 ID

            InformationActiveModel {
                id: item["id"].as_i64().map(|v| Set(v as i32)).unwrap_or(NotSet),
                name: Set(item["name"].as_str().unwrap_or("").to_string()),
                formula: Set(item["formula"].as_str().unwrap_or("").to_string()),
                cas_no: Set(item["casNo"].as_str().unwrap_or("").to_string()),
                number: Set(item["number"].as_i64().unwrap_or(0) as i32),
                internal_name: Set(item["internalName"].as_str().unwrap_or("").to_string()),
            }
        })
        .collect();

    // 3. 批量分段插入
    for chunk in models.chunks(500) {
        InformationEntity::insert_many(chunk.to_vec())
            .exec(&txn)
            .await?;
    }

    // 4. 重要：重置自增计数器，防止下次插入冲突
    // 根据你使用的数据库类型选择对应的 SQL
    let db_backend = txn.get_database_backend();
    let reset_sql: Option<String> = match db_backend {
        sea_orm::DatabaseBackend::MySql => Some(format!(
            "ALTER TABLE pp_component_information_entity AUTO_INCREMENT = {}",
            max_id + 1
        )),
        sea_orm::DatabaseBackend::Postgres => Some(format!(
            "SELECT setval(pg_get_serial_sequence('pp_component_information_entity', 'id'), {})",
            max_id
        )),
        sea_orm::DatabaseBackend::Sqlite => Some(format!(
            "UPDATE sqlite_sequence SET seq = {} WHERE name = 'pp_component_information_entity'",
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
        eprintln!("Warning: Unsupported database backend for resetting auto-increment.   ->pp_component_information_entity");
    }

    txn.commit().await?;
    Ok(())
}
