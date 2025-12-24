use crate::service_database::database_config::db_config_connection::get_config_db;

use napi_derive::napi;

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, Set};

use serde::{Deserialize, Serialize};
// ======================================
// 假设 ConfFunctionPic 的实体定义在这里
use crate::service_database::database_config::entity::conf_system_variable_entity::{
    ActiveModel as ConfSystemVariableActiveModel, Entity as ConfSystemVariableEntity,
    Model as ConfSystemVariableModel,
};

#[napi(
    object,
    namespace = "confSystemVariable",
    js_name = "ConfSystemVariableDto"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfSystemVariableDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub value: f64,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfSystemVariableModel> for ConfSystemVariableDto {
    fn from(model: ConfSystemVariableModel) -> Self {
        ConfSystemVariableDto {
            id: model.id,
            code: model.code,
            name: model.name,
            value: model.value,
        }
    }
}

pub async fn select_conf_system_variable_all() -> Result<Vec<ConfSystemVariableDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfSystemVariableEntity::find().all(db).await?;

    // 将查询结果 (Option<ConfSystemVariableModel>) 转换为 Option<ConfSystemVariableDto>
    let dto: Vec<ConfSystemVariableDto> = models
        .into_iter()
        .map(ConfSystemVariableDto::from)
        .collect();

    Ok(dto)
}
pub async fn upsert_and_insert_fixed_conf_system_variable_params(
    data_list: Vec<ConfSystemVariableDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfSystemVariableEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfSystemVariableActiveModel = model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.value = Set(config.value);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfSystemVariableActiveModel {
                id: NotSet,
                name: Set(config.name),
                value: Set(config.value),
                code: Set(config.code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}
