use crate::tool_handle::model_type::model_type::{NodeType, NodeTypeCategory};
use serde_json::{Map, Value, json};
use std::str::FromStr;

/// 【装箱】将动态对象拆分为固定字段 + initParams 字符串 (用于存入数据库)
pub fn pack_to_storage_handle(mut entity: Map<String, Value>) -> Map<String, Value> {
    // 1. 提取公共固定字段（这些字段在数据库有独立列）
    let id = entity.remove("id").unwrap_or(json!(""));
    let name = entity.remove("name").unwrap_or(json!(""));
    let r#type_val = entity.remove("type").unwrap_or(json!(""));
    let type_str = r#type_val.as_str().unwrap_or("");
    let model_id = entity.remove("modelId").unwrap_or(json!(""));
    let status = entity.remove("status").unwrap_or(json!(0));
    let actived = entity.remove("actived").unwrap_or(json!(1));

    // 2. 确定哪些字段该放入 initParams 对象
    let category: Option<NodeTypeCategory> = NodeType::from_str(type_str)
        .ok()
        .map(NodeTypeCategory::from_node);

    let init_params_obj = match category {
        // 物流：排除物性/脚本等独立字段，剩下的全放进 initParams
        Some(NodeTypeCategory::Material) => {
            let excludes = [
                "createAt",
                "script",
                "feed",
                "product",
                "pressureFixed",
                "flowFixed",
                "pfDisconnected",
                "flashType",
                "fluidPackage",
            ];
            for key in excludes {
                entity.remove(key);
            }
            Value::Object(entity)
        }
        // 能流：仅排除脚本字段
        Some(NodeTypeCategory::Energy) => {
            entity.remove("script");
            entity.remove("createAt");
            Value::Object(entity)
        }
        // 逻辑/传感器/脚本UO：仅保留 params 字段进入 initParams
        Some(NodeTypeCategory::Logic)
        | Some(NodeTypeCategory::ScriptUO)
        | Some(NodeTypeCategory::Sensor) => {
            json!({ "params": entity.get("params") })
        }
        // AI 模块：仅保留通讯字段
        Some(NodeTypeCategory::Aiuo) => {
            json!({ "commIn": entity.get("commIn"), "commOut": entity.get("commOut") })
        }
        // 默认情况：根据前缀判断或保留 params + holdups
        _ => {
            if NodeType::is_logic_node_from_str(type_str) {
                json!({ "params": entity.get("params") })
            } else {
                json!({ "params": entity.get("params"), "holdups": entity.get("holdups") })
            }
        }
    };

    // 3. 组装存入数据库的实体 Map
    let mut db_entity = Map::new();
    db_entity.insert("graphicId".to_string(), id);
    db_entity.insert("name".to_string(), name);
    db_entity.insert("type".to_string(), r#type_val);
    db_entity.insert("modelId".to_string(), model_id);
    db_entity.insert("status".to_string(), status);
    db_entity.insert("actived".to_string(), actived);
    // 关键点：将对象序列化为 JSON 字符串
    db_entity.insert("initParams".to_string(), json!(init_params_obj.to_string()));

    db_entity
}

/// 【拆箱/还原】将数据库中的 initParams 字符串解析并平铺回 Map 根部 (用于返回给前端)
pub fn unpack_from_storage_handle(mut db_entity: Map<String, Value>) -> Map<String, Value> {
    // 修复生命周期问题：通过 match 转移所有权
    if let Some(params_map) = db_entity
        .remove("initParams")
        .and_then(|v| v.as_str().map(|s| s.to_string())) // 1. 拿到 String 的所有权
        .and_then(|s| serde_json::from_str::<Value>(&s).ok()) // 2. 解析为 Value
        .and_then(|v| {
            // 3. 关键：将 Value 内部的 Object (Map) 提取出来，转移所有权
            if let Value::Object(m) = v {
                Some(m)
            } else {
                None
            }
        })
    {
        // 4. 将解析出来的键值对平铺回主 Map
        for (k, v) in params_map {
            db_entity.insert(k, v);
        }
    }

    db_entity
}
