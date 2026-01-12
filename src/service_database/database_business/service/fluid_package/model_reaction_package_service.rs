// use crate::service_database::database_business::db_business_connection::get_business_db;
// use sea_orm::{
//     ActiveModelTrait, FromQueryResult, QueryFilter, Set, TransactionTrait, entity::prelude::*,
// };
// // 假设这是反应包实体的路径
// use crate::service_database::database_business::entity::reaction_package::model_reaction_package_entity::{
//     Entity as ReactionEntity, Column as ReactionColumn, Model as ReactionModel, ActiveModel as ReactionActiveModel
// };
// // 假设这是反应包详情实体的路径（对应 TS 中的 getModelReactionDetailQueryBuilder）
// use crate::service_database::database_business::entity::reaction_package::model_reaction_detail_entity::{
//     Entity as DetailEntity, Column as DetailColumn
// };

// #[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
// #[napi(
//     object,
//     namespace = "modelReaction",
//     js_name = "ModelReactionPackageDTO"
// )]
// pub struct ModelReactionPackageDTO {
//     pub id: String,
//     pub reaction_name: String,
//     pub model_id: String,
//     pub compound_channel_id: String,
//     pub fluid_package_ids: String,
//     pub reaction_package_type: String,
// }

// // ======================================
// // 转换逻辑 (Impls)
// // ======================================

// impl From<ReactionModel> for ModelReactionPackageDTO {
//     fn from(m: ReactionModel) -> Self {
//         Self {
//             id: m.id,
//             reaction_name: m.reaction_name,
//             model_id: m.model_id,
//             compound_channel_id: m.compound_channel_id,
//             fluid_package_ids: m.fluid_package_ids,
//             reaction_package_type: m.reaction_package_type,
//         }
//     }
// }

// impl ModelReactionPackageDTO {
//     fn into_active_model(self) -> ReactionActiveModel {
//         ReactionActiveModel {
//             id: Set(self.id),
//             reaction_name: Set(self.reaction_name),
//             model_id: Set(self.model_id),
//             compound_channel_id: Set(self.compound_channel_id),
//             fluid_package_ids: Set(self.fluid_package_ids),
//             reaction_package_type: Set(self.reaction_package_type),
//         }
//     }
// }
