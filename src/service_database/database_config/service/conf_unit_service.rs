use crate::service_database::database_config::db_config_connection::get_config_db;

use napi_derive::napi;

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, NotSet, QueryFilter, Set,
};

use serde::{Deserialize, Serialize};

// 单位集合
use crate::service_database::database_config::entity::conf_unit_set_entity::{
    ActiveModel as ConfUnitSetActiveModel, Column as ConfUnitSetColumn,
    Entity as ConfUnitSetEntity, Model as ConfUnitSetModel,
};

#[napi(object, namespace = "confUnit")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfUnitSetDto {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub en_name: String,
    pub status: u8,
    pub is_default: u8,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfUnitSetModel> for ConfUnitSetDto {
    fn from(model: ConfUnitSetModel) -> Self {
        ConfUnitSetDto {
            id: model.id,
            code: model.code,
            name: model.name,
            en_name: model.en_name,
            status: model.status,
            is_default: model.is_default,
        }
    }
}

pub async fn select_conf_unit_set_one(code: String) -> Result<ConfUnitSetDto, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitSetEntity::find()
        .filter(ConfUnitSetColumn::Code.eq(code))
        .one(db)
        .await?;

    let dto: ConfUnitSetDto = models.map(ConfUnitSetDto::from).unwrap();

    Ok(dto)
}

pub async fn select_conf_unit_set_all() -> Result<Vec<ConfUnitSetDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitSetEntity::find()
        .filter(ConfUnitSetColumn::Status.eq(0))
        .all(db)
        .await?;

    let dto: Vec<ConfUnitSetDto> = models.into_iter().map(ConfUnitSetDto::from).collect();

    Ok(dto)
}
pub async fn upsert_and_insert_conf_unit_set(data_list: Vec<ConfUnitSetDto>) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfUnitSetEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> =
        existing_models.into_iter().map(|m| (m.id, m)).collect();
    let mut success_count: i32 = 0;
    // id,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.id) {
            // 更新逻辑
            let mut active_model: ConfUnitSetActiveModel = model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.en_name = Set(config.en_name);
            active_model.status = Set(config.status);
            active_model.is_default = Set(config.is_default);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfUnitSetActiveModel {
                id: Set(config.id),
                name: Set(config.name),
                en_name: Set(config.en_name),
                status: Set(config.status),
                is_default: Set(config.is_default),
                code: Set(config.code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}

// 单位类型
use crate::service_database::database_config::entity::conf_unit_item_entity::{
    ActiveModel as ConfUnitItemActiveModel, Column as ConfUnitItemColumn,
    Entity as ConfUnitItemEntity, Model as ConfUnitItemModel,
};

#[napi(object, namespace = "confUnit")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfUnitItemDto {
    pub id: i32,
    pub code: String,
    pub value: String,
    pub set_code: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfUnitItemModel> for ConfUnitItemDto {
    fn from(model: ConfUnitItemModel) -> Self {
        ConfUnitItemDto {
            id: model.id,
            code: model.code,
            value: model.value,
            set_code: model.set_code,
        }
    }
}

pub async fn select_conf_unit_item_all(set_code: String) -> Result<Vec<ConfUnitItemDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitItemEntity::find()
        .filter(ConfUnitItemColumn::SetCode.eq(set_code))
        .all(db)
        .await?;

    let dto: Vec<ConfUnitItemDto> = models.into_iter().map(ConfUnitItemDto::from).collect();

    Ok(dto)
}

pub async fn select_conf_unit_item_all_by_codes(
    set_code: String,
    codes: Vec<String>,
) -> Result<Vec<ConfUnitItemDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitItemEntity::find()
        .filter(ConfUnitItemColumn::SetCode.eq(set_code))
        .filter(ConfUnitItemColumn::Code.is_in(codes))
        .all(db)
        .await?;

    let dto: Vec<ConfUnitItemDto> = models.into_iter().map(ConfUnitItemDto::from).collect();

    Ok(dto)
}

pub async fn upsert_and_insert_conf_unit_item(
    data_list: Vec<ConfUnitItemDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfUnitItemEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> =
        existing_models.into_iter().map(|m| (m.id, m)).collect();
    let mut success_count: i32 = 0;
    // 根据id,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.id) {
            // 更新逻辑
            let mut active_model: ConfUnitItemActiveModel = model.clone().into_active_model();
            active_model.code = Set(config.code);
            active_model.value = Set(config.value);
            active_model.set_code = Set(config.set_code);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfUnitItemActiveModel {
                id: Set(config.id),
                code: Set(config.code),
                value: Set(config.value),
                set_code: Set(config.set_code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}

// 单位类型
use crate::service_database::database_config::entity::conf_unit_first_category_entity::{
    ActiveModel as ConfUnitFirstCategoryActiveModel, Entity as ConfUnitFirstCategoryEntity,
    Model as ConfUnitFirstCategoryModel,
};

#[napi(object, namespace = "confUnit")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfUnitFirstCategoryDto {
    pub code: String,
    pub name: String,
    pub en_name: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfUnitFirstCategoryModel> for ConfUnitFirstCategoryDto {
    fn from(model: ConfUnitFirstCategoryModel) -> Self {
        ConfUnitFirstCategoryDto {
            code: model.code,
            name: model.name,
            en_name: model.en_name,
        }
    }
}

pub async fn select_conf_unit_first_category_all() -> Result<Vec<ConfUnitFirstCategoryDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitFirstCategoryEntity::find().all(db).await?;

    let dto: Vec<ConfUnitFirstCategoryDto> = models
        .into_iter()
        .map(ConfUnitFirstCategoryDto::from)
        .collect();

    Ok(dto)
}

pub async fn upsert_and_insert_conf_unit_first_category(
    data_list: Vec<ConfUnitFirstCategoryDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfUnitFirstCategoryEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfUnitFirstCategoryActiveModel =
                model.clone().into_active_model();
            active_model.code = Set(config.code);
            active_model.name = Set(config.name);
            active_model.en_name = Set(config.en_name);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfUnitFirstCategoryActiveModel {
                id: NotSet,
                code: Set(config.code),
                name: Set(config.name),
                en_name: Set(config.en_name),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}

use crate::service_database::database_config::entity::conf_unit_second_category_entity::{
    ActiveModel as ConfUnitSecondCategoryActiveModel, Entity as ConfUnitSecondCategoryEntity,
    Model as ConfUnitSecondCategoryModel,
};

#[napi(object, namespace = "confUnit")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfUnitSecondCategoryDto {
    pub code: String,
    pub name: String,
    pub en_name: String,
    pub category_first_code: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfUnitSecondCategoryModel> for ConfUnitSecondCategoryDto {
    fn from(model: ConfUnitSecondCategoryModel) -> Self {
        ConfUnitSecondCategoryDto {
            code: model.code,
            name: model.name,
            en_name: model.en_name,
            category_first_code: model.category_first_code,
        }
    }
}

pub async fn select_conf_unit_second_category_all() -> Result<Vec<ConfUnitSecondCategoryDto>, DbErr>
{
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitSecondCategoryEntity::find()
        // .filter(ConfUnitSecondCategoryColumn::CategoryFirstCode.eq(first_code))
        .all(db)
        .await?;
    let dto: Vec<ConfUnitSecondCategoryDto> = models
        .into_iter()
        .map(ConfUnitSecondCategoryDto::from)
        .collect();

    Ok(dto)
}

pub async fn upsert_and_insert_conf_unit_second_category(
    data_list: Vec<ConfUnitSecondCategoryDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfUnitSecondCategoryEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfUnitSecondCategoryActiveModel =
                model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.en_name = Set(config.en_name);
            active_model.category_first_code = Set(config.category_first_code);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfUnitSecondCategoryActiveModel {
                id: NotSet,
                code: Set(config.code),
                name: Set(config.name),
                en_name: Set(config.en_name),
                category_first_code: Set(config.category_first_code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}

use crate::service_database::database_config::entity::conf_unit_item_category_entity::{
    ActiveModel as ConfUnitItemCategoryActiveModel, Entity as ConfUnitItemCategoryEntity,
    Model as ConfUnitItemCategoryModel,
};

#[napi(object, namespace = "confUnit")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfUnitItemCategoryDto {
    pub code: String,
    pub name: String,
    pub en_name: String,
    pub category_second_code: String,
}

// 假设存在 Model 到 DTO 的转换
impl From<ConfUnitItemCategoryModel> for ConfUnitItemCategoryDto {
    fn from(model: ConfUnitItemCategoryModel) -> Self {
        ConfUnitItemCategoryDto {
            code: model.code,
            name: model.name,
            en_name: model.en_name,
            category_second_code: model.category_second_code,
        }
    }
}

pub async fn select_conf_unit_item_category_all() -> Result<Vec<ConfUnitItemCategoryDto>, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接

    let models = ConfUnitItemCategoryEntity::find()
        // .filter(ConfUnitItemCategoryColumn::CategorySecondCode.eq(second_code))
        .all(db)
        .await?;
    let dto: Vec<ConfUnitItemCategoryDto> = models
        .into_iter()
        .map(ConfUnitItemCategoryDto::from)
        .collect();

    Ok(dto)
}

pub async fn upsert_and_insert_conf_unit_item_category(
    data_list: Vec<ConfUnitItemCategoryDto>,
) -> Result<i32, DbErr> {
    let db = get_config_db().await.unwrap(); // 获取数据库连接
    let existing_models = ConfUnitItemCategoryEntity::find().all(db).await?;
    let existing_code_map: std::collections::HashMap<_, _> = existing_models
        .into_iter()
        .map(|m| (m.code.clone(), m))
        .collect();
    let mut success_count: i32 = 0;
    // 根据code,本身数据库有数据的进行更新，没有的进行插入
    for config in data_list {
        if let Some(model) = existing_code_map.get(&config.code) {
            // 更新逻辑
            let mut active_model: ConfUnitItemCategoryActiveModel =
                model.clone().into_active_model();
            active_model.name = Set(config.name);
            active_model.en_name = Set(config.en_name);
            active_model.category_second_code = Set(config.category_second_code);
            active_model.update(db).await?;
        } else {
            // 插入逻辑
            let active_model = ConfUnitItemCategoryActiveModel {
                id: NotSet,
                code: Set(config.code),
                name: Set(config.name),
                en_name: Set(config.en_name),
                category_second_code: Set(config.category_second_code),
            };
            active_model.insert(db).await?;
        }

        success_count += 1;
    }
    Ok(success_count)
}
