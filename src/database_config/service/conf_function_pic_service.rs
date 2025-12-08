use crate::database_config::db_config_connection::get_config_db;
use napi_derive::napi;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter, Set,
};

use serde::{Deserialize, Serialize};
// ======================================
// 假设 ConfFunctionPic 的实体定义在这里
use crate::database_config::entity::conf_function_pic::{
    ActiveModel as ConfFunctionPicActiveModel,
    Column as ConfFunctionPicColumn, // 需要引入 Column 枚举来按 code 查询
    Entity as ConfFunctionPicEntity,
    Model as ConfFunctionPicModel,
};

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionPicDto {
    pub id: i32,
    pub name: String,
    pub picture: String,
    pub code: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfFunctionPicModel> for FunctionPicDto {
    fn from(model: ConfFunctionPicModel) -> Self {
        FunctionPicDto {
            id: model.id,
            name: model.name,
            picture: model.picture,
            code: model.code,
        }
    }
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewFunctionPicDto {
    pub name: String,
    pub picture: String,
    pub code: String,
}

pub async fn select_conf_function_pic_by_code(query_code: String) -> Result<FunctionPicDto, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let model = ConfFunctionPicEntity::find()
        .filter(ConfFunctionPicColumn::Code.eq(&query_code))
        .filter(ConfFunctionPicColumn::Name.eq(&query_code))
        .one(db)
        .await?;

    // 将查询结果 (Option<ConfFunctionPicModel>) 转换为 Option<FunctionPicDto>
    let dto = model.map(FunctionPicDto::from).unwrap();

    Ok(dto)
}
pub async fn upsert_and_insert_fixed_conf_pic(
    pic_datas: Vec<NewFunctionPicDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let mut success_count: i32 = 0;

    for config in pic_datas {
        // 1. 尝试根据 code 查找现有记录
        let existing_model = ConfFunctionPicEntity::find()
            .filter(ConfFunctionPicColumn::Code.eq(&config.code))
            .one(db)
            .await?;

        match existing_model {
            Some(model) => {
                // --- 📌 更新 (Update) 逻辑：记录存在 ---

                // 转换为 ActiveModel
                let mut active_model: ConfFunctionPicActiveModel = model.into_active_model();

                // 设置需要更新的字段
                active_model.name = Set(config.code);
                active_model.picture = Set(config.picture);
                // code 字段通常保持不变，但也可以 Set(config.code)

                // 执行更新
                active_model.update(db).await?;
            }
            None => {
                // --- ➕ 插入 (Insert) 逻辑：记录不存在 ---

                // 构造新的 ActiveModel
                let active_model = ConfFunctionPicActiveModel {
                    id: sea_orm::NotSet, // ID 由数据库自动生成
                    name: Set(config.name),
                    picture: Set(config.picture),
                    code: Set(config.code),
                };

                // 执行插入
                active_model.insert(db).await?;
            }
        }
        success_count += 1;
    }

    Ok(success_count)
}
