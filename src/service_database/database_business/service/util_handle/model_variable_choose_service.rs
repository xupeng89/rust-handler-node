use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_util_handle::model_variable_choose_entity::{
    ActiveModel as ChooseActiveModel, Column as ChooseColumn, Entity as ChooseEntity,
    Model as ChooseModel,
};
use napi_derive::napi;
use sea_orm::{ActiveModelTrait, QueryFilter, QueryOrder, FromQueryResult,Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
#[napi(
    object,
    namespace = "modelVariable",
    js_name = "ModelVariableChooseDTO"
)]
pub struct ModelVariableChooseDTO {
    pub id: String,
    pub r#type: String,
    pub model_id: String,
    pub graphic_source_id: String,
    pub connect_material_type: String,
    pub show_name: String,
    pub filter_msg: String,
    pub unit_code: String,
    pub graphic_type: String,
    pub from_type: String,
    pub data_type: String,
    pub data_mode: String,
    pub dcs_name: String,
    pub value_name: String,
    pub sort: i32,
    pub y_max_value: Option<i32>,
    pub y_min_value: Option<i32>,
    pub information: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelVariable",
    js_name = "ModelVariableChooseUpdateDTO"
)]
pub struct ModelVariableChooseUpdateDTO {
    pub id: String,
    pub model_id: String,
    pub r#type: Option<String>,
    pub graphic_source_id: Option<String>,
    pub connect_material_type: Option<String>,
    pub show_name: Option<String>,
    pub filter_msg: Option<String>,
    pub unit_code: Option<String>,
    pub graphic_type: Option<String>,
    pub from_type: Option<String>,
    pub data_type: Option<String>,
    pub data_mode: Option<String>,
    pub dcs_name: Option<String>,
    pub value_name: Option<String>,
    pub sort: Option<i32>,
    pub y_max_value: Option<i32>,
    pub y_min_value: Option<i32>,
    pub information: Option<String>,
}

impl From<ChooseModel> for ModelVariableChooseDTO {
    fn from(m: ChooseModel) -> Self {
        Self {
            id: m.id,
            r#type: m.r#type,
            model_id: m.model_id,
            graphic_source_id: m.graphic_source_id,
            connect_material_type: m.connect_material_type,
            show_name: m.show_name,
            filter_msg: m.filter_msg,
            unit_code: m.unit_code,
            graphic_type: m.graphic_type,
            from_type: m.from_type,
            data_type: m.data_type,
            data_mode: m.data_mode,
            dcs_name: m.dcs_name,
            value_name: m.value_name,
            sort: m.sort,
            y_max_value: m.y_max_value,
            y_min_value: m.y_min_value,
            information: m.information,
        }
    }
}

impl ModelVariableChooseDTO {
    fn into_active_model(self) -> ChooseActiveModel {
        ChooseActiveModel {
            id: Set(self.id),
            r#type: Set(self.r#type),
            model_id: Set(self.model_id),
            graphic_source_id: Set(self.graphic_source_id),
            connect_material_type: Set(self.connect_material_type),
            show_name: Set(self.show_name),
            filter_msg: Set(self.filter_msg),
            unit_code: Set(self.unit_code),
            graphic_type: Set(self.graphic_type),
            from_type: Set(self.from_type),
            data_type: Set(self.data_type),
            data_mode: Set(self.data_mode),
            dcs_name: Set(self.dcs_name),
            value_name: Set(self.value_name),
            sort: Set(self.sort),
            y_max_value: Set(self.y_max_value),
            y_min_value: Set(self.y_min_value),
            information: Set(self.information),
        }
    }
}

pub async fn insert(data: ModelVariableChooseDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let am = data.into_active_model();
    ChooseEntity::insert(am).exec(db).await?;
    Ok(true)
}

pub async fn get_by_id_and_model(
    id: String,
    model_id: String,
) -> Result<Option<ModelVariableChooseDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::find()
        .filter(ChooseColumn::Id.eq(id))
        .filter(ChooseColumn::ModelId.eq(model_id))
        .one(db)
        .await?;
    Ok(res.map(ModelVariableChooseDTO::from))
}

pub async fn get_by_model_and_type(
    model_id: String,
    r#type: String,
) -> Result<Vec<ModelVariableChooseDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::find()
        .filter(ChooseColumn::ModelId.eq(model_id))
        .filter(ChooseColumn::Type.eq(r#type))
        .order_by_asc(ChooseColumn::Sort)
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelVariableChooseDTO::from).collect())
}

pub async fn get_by_model_like_type(
    model_id: String,
    r#type: String,
) -> Result<Vec<ModelVariableChooseDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::find()
        .filter(ChooseColumn::ModelId.eq(model_id))
        .filter(ChooseColumn::Type.contains(&r#type)) // LIKE %type%
        .order_by_asc(ChooseColumn::Sort)
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelVariableChooseDTO::from).collect())
}

pub async fn delete_by_id(id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::delete_by_id(id).exec(db).await?;
    Ok(res.rows_affected > 0)
}

pub async fn delete_batch(
    ids: Vec<String>,
    model_id: String,
    r#type: String,
) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::delete_many()
        .filter(ChooseColumn::Id.is_in(ids))
        .filter(ChooseColumn::ModelId.eq(model_id))
        .filter(ChooseColumn::Type.eq(r#type))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}

pub async fn delete_by_types(types: Vec<String>, model_id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = ChooseEntity::delete_many()
        .filter(ChooseColumn::Type.is_in(types))
        .filter(ChooseColumn::ModelId.eq(model_id))
        .exec(db)
        .await?;
    Ok(res.rows_affected > 0)
}

pub async fn update(data: ModelVariableChooseUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = ChooseEntity::find()
        .filter(ChooseColumn::Id.eq(data.id.clone()))
        .filter(ChooseColumn::ModelId.eq(data.model_id.clone()))
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Variable choose not found".into()))?;

    let mut am: ChooseActiveModel = existing.into();
    if let Some(v) = data.r#type {
        am.r#type = Set(v);
    }
    if let Some(v) = data.graphic_source_id {
        am.graphic_source_id = Set(v);
    }
    if let Some(v) = data.connect_material_type {
        am.connect_material_type = Set(v);
    }
    if let Some(v) = data.show_name {
        am.show_name = Set(v);
    }
    if let Some(v) = data.filter_msg {
        am.filter_msg = Set(v);
    }
    if let Some(v) = data.unit_code {
        am.unit_code = Set(v);
    }
    if let Some(v) = data.graphic_type {
        am.graphic_type = Set(v);
    }
    if let Some(v) = data.from_type {
        am.from_type = Set(v);
    }
    if let Some(v) = data.data_type {
        am.data_type = Set(v);
    }
    if let Some(v) = data.data_mode {
        am.data_mode = Set(v);
    }
    if let Some(v) = data.dcs_name {
        am.dcs_name = Set(v);
    }
    if let Some(v) = data.value_name {
        am.value_name = Set(v);
    }
    if let Some(v) = data.sort {
        am.sort = Set(v);
    }
    am.y_max_value = Set(data.y_max_value);
    am.y_min_value = Set(data.y_min_value);
    if let Some(v) = data.information {
        am.information = Set(v);
    }

    am.update(db).await?;
    Ok(true)
}
