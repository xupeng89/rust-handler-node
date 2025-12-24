use crate::service_database::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, QueryFilter, Set,
};

use serde::{Deserialize, Serialize};
// ======================================
// 假设 ConfFunctionPic 的实体定义在这里
use crate::service_database::database_config::entity::conf_function_pic_entity::{
    ActiveModel as ConfFunctionPicActiveModel,
    Column as ConfFunctionPicColumn, // 需要引入 Column 枚举来按 code 查询
    Entity as ConfFunctionPicEntity,
    Model as ConfFunctionPicModel,
};

#[napi(object, namespace = "confFunctionPic", js_name = "FunctionPicDTO")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionPicDTO {
    pub id: i32,
    pub name: String,
    pub picture: String,
    pub code: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfFunctionPicModel> for FunctionPicDTO {
    fn from(model: ConfFunctionPicModel) -> Self {
        FunctionPicDTO {
            id: model.id,
            name: model.name,
            picture: model.picture,
            code: model.code,
        }
    }
}

#[napi(object, namespace = "confFunctionPic", js_name = "NewFunctionPicDTO")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFunctionPicDTO {
    pub name: String,
    pub picture: String,
    pub code: String,
}

pub async fn select_conf_function_pic_by_code(query_code: String) -> Result<FunctionPicDTO, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let model = ConfFunctionPicEntity::find()
        .filter(ConfFunctionPicColumn::Code.eq(&query_code))
        .filter(ConfFunctionPicColumn::Name.eq(&query_code))
        .one(db)
        .await?;

    // 将查询结果 (Option<ConfFunctionPicModel>) 转换为 Option<FunctionPicDto>
    let dto = model.map(FunctionPicDTO::from).unwrap();

    Ok(dto)
}
pub async fn upsert_and_insert_fixed_conf_pic(
    pic_datas: Vec<NewFunctionPicDTO>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    if pic_datas.is_empty() {
        return Ok(0);
    }
    // 2. 批量提取所有 code，一次性查询现有记录（减少数据库查询次数）
    let target_codes: Vec<_> = pic_datas.iter().map(|d| d.code.clone()).collect();
    let existing_models = ConfFunctionPicEntity::find()
        .filter(ConfFunctionPicColumn::Code.is_in(target_codes))
        .all(db)
        .await?;

    // 3. 构建 code → 模型的映射（O(1) 查找）
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();

    let mut success_count: i32 = 0;

    for config in pic_datas {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfFunctionPicActiveModel = model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.picture = Set(config.picture);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfFunctionPicActiveModel {
                id: NotSet,
                name: Set(config.name),
                picture: Set(config.picture),
                code: Set(config.code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }

    Ok(success_count)
}
