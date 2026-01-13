use crate::service_database::database_business::db_business_connection::get_business_db;
use crate::service_database::database_business::entity::model_config::model_graphic_page_entity::{
    ActiveModel as GraphicActiveModel, Column as GraphicColumn, Entity as GraphicEntity,
    Model as GraphicModel,
};
use napi_derive::napi;
use sea_orm::{ActiveModelTrait, QueryFilter, Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(object, namespace = "modelFlowSheet", js_name = "ModelGraphicPageDTO")]
pub struct ModelGraphicPageDTO {
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub scale: String,
    pub translate: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[napi(
    object,
    namespace = "modelFlowSheet",
    js_name = "ModelFlowSheetUpdateDTO"
)]
pub struct ModelFlowSheetUpdateDTO {
    pub id: String,
    pub model_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scale: Option<String>,
    pub translate: Option<String>, // 接收已转为 String 的值
}

impl From<GraphicModel> for ModelGraphicPageDTO {
    fn from(m: GraphicModel) -> Self {
        Self {
            id: m.id,
            model_id: m.model_id,
            name: m.name,
            description: m.description,
            scale: m.scale,
            translate: m.translate,
        }
    }
}

impl ModelGraphicPageDTO {
    fn into_active_model(self) -> GraphicActiveModel {
        GraphicActiveModel {
            id: Set(self.id),
            model_id: Set(self.model_id),
            name: Set(self.name),
            description: Set(self.description),
            scale: Set(self.scale),
            translate: Set(self.translate),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[napi(
    object,
    namespace = "modelFlowSheet",
    js_name = "ModelGraphicPageUpdateDTO"
)]
pub struct ModelGraphicPageUpdateDTO {
    pub id: String,
    pub model_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub scale: Option<String>,
    pub translate: Option<String>,
}

pub async fn get_one_by_id(id: String) -> Result<Option<ModelGraphicPageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = GraphicEntity::find_by_id(id).one(db).await?;
    Ok(res.map(ModelGraphicPageDTO::from))
}

pub async fn get_list_by_model_id(model_id: String) -> Result<Vec<ModelGraphicPageDTO>, DbErr> {
    let db = get_business_db().await?;
    let res = GraphicEntity::find()
        .filter(GraphicColumn::ModelId.eq(model_id))
        .all(db)
        .await?;
    Ok(res.into_iter().map(ModelGraphicPageDTO::from).collect())
}

pub async fn create(data: ModelGraphicPageDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let am = data.into_active_model();
    GraphicEntity::insert(am).exec(db).await?;
    Ok(true)
}

pub async fn update(data: ModelFlowSheetUpdateDTO) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let existing = GraphicEntity::find_by_id(data.id.clone())
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("Flowsheet page not found".into()))?;

    let mut am: GraphicActiveModel = existing.into();

    if let Some(v) = data.model_id {
        am.model_id = Set(v);
    }
    if let Some(v) = data.name {
        am.name = Set(v);
    }
    if let Some(v) = data.description {
        am.description = Set(v);
    }
    if let Some(v) = data.scale {
        am.scale = Set(v);
    }
    if let Some(v) = data.translate {
        am.translate = Set(v);
    }

    am.update(db).await?;
    Ok(true)
}

pub async fn delete_by_id(id: String) -> Result<bool, DbErr> {
    let db = get_business_db().await?;
    let res = GraphicEntity::delete_by_id(id).exec(db).await?;
    Ok(res.rows_affected > 0)
}
