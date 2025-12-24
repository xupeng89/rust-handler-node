use crate::service_database::database_physical_property::db_physical_property_connection::get_physical_property_db;
use crate::service_database::database_physical_property::entity::physical_calc_function_property_entity::{Entity as FunctionEntity,
    Model as FunctionModel, Column as FunctionColumn, ActiveModel as FunctionActiveModel};

use crate::service_database::database_physical_property::entity::physical_calc_relation_property_entity::{Entity as RelationEntity,
    ActiveModel as RelationActiveModel, Column as RelationColumn};
use crate::service_database::database_physical_property::entity::physical_calc_base_property_entity::{Entity as BasePropertyEntity,
    ActiveModel as BasePropertyActiveModel, Column as BasePropertyColumn};

use crate::sync_physical_calc_data;
use crate::tool_handle::result_entity::FunctionOptionDTO;
use napi_derive::napi;
use sea_orm::{entity::prelude::*, FromQueryResult, JoinType, QuerySelect, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[napi(object, namespace = "physicalCalc", js_name = "CalcFunctionDTO")]
pub struct CalcFunctionDTO {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub args_json: String,
    pub is_show: i32,
}

// 类型转换
impl From<FunctionModel> for CalcFunctionDTO {
    fn from(ele: FunctionModel) -> Self {
        CalcFunctionDTO {
            id: ele.id,
            name: ele.name,
            code: ele.code,
            args_json: ele.args_json,
            is_show: ele.is_show,
        }
    }
}

/// 定一个整体联合查询的返回结果
#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(object, namespace = "physicalCalc", js_name = "CalcAllDetailDTO")]
pub struct CalcAllDetailDTO {
    // BaseProperty 表字段 (起别名)
    pub base_property_code: String,
    pub base_property_id: i32,
    pub base_property_name: String,
    pub base_property_key: String,
    pub base_property_phase: String,
    pub base_property_mixture: i32,
    // Function 表字段 (起别名)
    pub calc_function_code: String,
}

// 读取所有方法值数据
pub async fn get_pp_calc_function_list() -> Result<Vec<CalcFunctionDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let calc_function_list: Vec<FunctionModel> = FunctionEntity::find().all(db).await?;
    let result = calc_function_list
        .into_iter()
        .map(CalcFunctionDTO::from)
        .collect();
    Ok(result)
}

// 根据 IDs 获取所有方法
pub async fn get_all_pp_calc_function_by_ids(ids: Vec<i32>) -> Result<Vec<CalcFunctionDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let calc_function_list: Vec<FunctionModel> = FunctionEntity::find()
        .filter(FunctionColumn::Id.is_in(ids))
        .all(db)
        .await?;
    let result = calc_function_list
        .into_iter()
        .map(CalcFunctionDTO::from)
        .collect();
    Ok(result)
}

// 根据 Codes 获取
pub async fn get_first_pp_calc_function_by_codes(
    codes: Vec<String>,
) -> Result<Vec<CalcFunctionDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    let calc_function_list = FunctionEntity::find()
        .filter(FunctionColumn::Code.is_in(codes))
        .all(db)
        .await?;
    let result = calc_function_list
        .into_iter()
        .map(CalcFunctionDTO::from)
        .collect();
    Ok(result)
}

// 关联查询：根据 BasePhysicalId 获取 Function
pub async fn get_pp_function_options_by_bp_id(bp_id: i32) -> Result<Vec<FunctionOptionDTO>, DbErr> {
    let db = get_physical_property_db().await?;
    RelationEntity::find()
        .select_only()
        // 注意：这里的 alias ("label" 和 "value") 必须与 DTO 结构体的字段名一致
        .column_as(FunctionColumn::Name, "label")
        .column_as(FunctionColumn::Code, "value")
        // 手动 Join
        .join(
            JoinType::InnerJoin,
            RelationEntity::belongs_to(FunctionEntity)
                .from(RelationColumn::FunctionId)
                .to(FunctionColumn::Id)
                .into(),
        )
        .filter(RelationColumn::BasePhysicalId.eq(bp_id))
        // 关键点：去重，防止关系表中有重复记录导致结果重复
        .distinct()
        // 映射到 DTO
        .into_model::<FunctionOptionDTO>()
        .all(db)
        .await
}

pub async fn get_fluid_package_all_bp_by_function_id(
    function_id: i32,
) -> Result<Vec<CalcAllDetailDTO>, DbErr> {
    let db = get_physical_property_db().await?;

    RelationEntity::find()
        .select_only()
        // --- 映射 BaseProperty 表字段到 DTO ---
        .column_as(BasePropertyColumn::Id, "base_property_id")
        .column_as(BasePropertyColumn::Code, "base_property_code")
        .column_as(BasePropertyColumn::Name, "base_property_name")
        .column_as(BasePropertyColumn::Key, "base_property_key")
        .column_as(BasePropertyColumn::Phase, "base_property_phase")
        .column_as(BasePropertyColumn::Mixture, "base_property_mixture")
        // --- 映射 Function 表字段到 DTO ---
        .column_as(FunctionColumn::Code, "calc_function_code")
        // 手动 Left Join BaseProperty (物理量表)
        .join_rev(
            JoinType::LeftJoin,
            BasePropertyEntity::belongs_to(RelationEntity)
                .from(BasePropertyColumn::Id)
                .to(RelationColumn::BasePhysicalId)
                .into(),
        )
        // 手动 Left Join Function (方法表 - 关联默认方法)
        .join_rev(
            JoinType::LeftJoin,
            FunctionEntity::belongs_to(RelationEntity)
                .from(FunctionColumn::Id)
                .to(RelationColumn::DefaultFunctionId)
                .into(),
        )
        // 过滤
        .filter(RelationColumn::FunctionId.eq(function_id))
        // 映射到 DTO 模型而非 JSON
        .into_model::<CalcAllDetailDTO>()
        .all(db)
        .await
}

// 初始化/覆盖数据 (Delete & Insert)
pub async fn init_pp_calc_all_msg_data(data_type: &str, data: Vec<Value>) -> Result<(), DbErr> {
    let db = get_physical_property_db().await?;
    let txn = db.begin().await?;

    // 1. 提取传入数据中所有的 ID
    let incoming_ids: Vec<i32> = data
        .iter()
        .filter_map(|item| item["id"].as_i64().map(|v| v as i32))
        .collect();

    // 2. 处理同步逻辑
    match data_type {
        "basePhysical" => {
            sync_physical_calc_data!(
                &txn,
                incoming_ids,
                data,
                BasePropertyEntity,
                BasePropertyActiveModel,
                item,
                am,
                {
                    am.name = Set(item["name"].as_str().unwrap_or("").to_string());
                    am.code = Set(item["code"].as_str().unwrap_or("").to_string());
                    am.key = Set(item["key"].as_str().unwrap_or("").to_string());
                    am.type_str = Set(item["type"].as_str().unwrap_or("").to_string());
                    am.phase = Set(item["phase"].as_str().unwrap_or("").to_string());
                    am.mixture = Set(item["mixture"].as_i64().unwrap_or(0) as i32);
                }
            );
        }
        "function" => {
            sync_physical_calc_data!(
                &txn,
                incoming_ids,
                data,
                FunctionEntity,
                FunctionActiveModel,
                item,
                am,
                {
                    let args_json_str = if item["argsJson"].is_string() {
                        item["argsJson"].as_str().unwrap().to_string()
                    } else {
                        item["argsJson"].to_string()
                    };
                    am.name = Set(item["name"].as_str().unwrap_or("").to_string());
                    am.code = Set(item["code"].as_str().unwrap_or("").to_string());
                    am.args_json = Set(args_json_str);
                    am.is_show = Set(item["isShow"].as_i64().unwrap_or(1) as i32);
                }
            );
        }
        "relation" => {
            sync_physical_calc_data!(
                &txn,
                incoming_ids,
                data,
                RelationEntity,
                RelationActiveModel,
                item,
                am,
                {
                    am.base_physical_id = Set(item["basePhysicalId"].as_i64().unwrap_or(0) as i32);
                    am.function_id = Set(item["functionId"].as_i64().unwrap_or(0) as i32);
                    am.default_function_id =
                        Set(item["defaultFunctionId"].as_i64().unwrap_or(0) as i32);
                }
            );
        }
        _ => return Err(DbErr::Custom("Unknown data type".to_string())),
    }

    txn.commit().await?;
    Ok(())
}
