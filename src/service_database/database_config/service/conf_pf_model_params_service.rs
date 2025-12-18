use crate::service_database::database_config::db_config_connection::get_config_db;

use napi_derive::napi;

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, Set};

use serde::{Deserialize, Serialize};
// ======================================
// 假设 ConfFunctionPic 的实体定义在这里
use crate::service_database::database_config::entity::conf_pf_model_params_entity::{
    ActiveModel as ConfPfModelParamsActiveModel, Entity as ConfPfModelParamsEntity,
    Model as ConfPfModelParamsModel,
};

#[napi(object, namespace = "confPfModelParams")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfPfModelParamsDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub solver_type: String,
    pub params: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfPfModelParamsModel> for ConfPfModelParamsDto {
    fn from(model: ConfPfModelParamsModel) -> Self {
        ConfPfModelParamsDto {
            id: model.id,
            code: model.code,
            name: model.name,
            solver_type: model.solver_type,
            params: model.params,
        }
    }
}

pub async fn select_conf_pf_model_params_all() -> Result<Vec<ConfPfModelParamsDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let model = ConfPfModelParamsEntity::find().all(db).await?;

    // 将查询结果 (Option<ConfPfModelParamsModel>) 转换为 Option<ConfPfModelParamsDto>
    let dto: Vec<ConfPfModelParamsDto> =
        model.into_iter().map(ConfPfModelParamsDto::from).collect();

    Ok(dto)
}
pub async fn upsert_and_insert_fixed_conf_pf_model_params(
    data: Vec<ConfPfModelParamsDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfPfModelParamsEntity::find().all(db).await?;

    // 检查是否存在相同的 code
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    for item in data {
        if let Some(existing_model) = existing_code_map.get(&item.code) {
            // 更新逻辑
            let mut active_model: ConfPfModelParamsActiveModel =
                existing_model.clone().into_active_model();
            active_model.name = Set(item.name);
            active_model.solver_type = Set(item.solver_type);
            active_model.params = Set(item.params);

            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfPfModelParamsActiveModel {
                id: NotSet,
                code: Set(item.code),
                name: Set(item.name),
                solver_type: Set(item.solver_type),
                params: Set(item.params),
            };
            active_model.insert(db).await?;
        }
        success_count += 1;
    }
    Ok(success_count)
}
