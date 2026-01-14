use napi_derive::napi;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumString};
#[napi(namespace = "modelType")]
pub mod model_type {
    use super::*;

    /// 节点类型核心枚举
    #[napi(string_enum)]
    #[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, EnumString, AsRefStr)]
    pub enum NodeType {
        Material,
        Energy,
        Valve,
        MixerTee,
        Seperator,
        SimpleDistTower,
        Pump,
        HeatX,
        AirCooler,
        ConversionReactor,
        Furnace,
        Pipe,
        CentrifugalCompressor,
        Signal,
        ByPass,
        Vote,
        Rs,
        Not,
        AndOr,
        Delay,
        Pulse,
        PositionInformation,
        Aiuo,
        TSensor,
        PSensor,
        LSensor,
        CompSensor,
        FSensor,
        NvfSensor,
        VfSensor,
        DSensor,
        Cutter,
        Rect,
        Ellipse,
        Path,
        Text,
        ScriptLogic,
        ScriptUO,
        CustomCustomerUO,
        CustomUserUO,
        CustomSpecialUO,
    }

    /// 图形/连线分类
    #[napi(string_enum)]
    #[derive(Serialize, Deserialize, AsRefStr, Display, EnumString)]
    pub enum GraphType {
        Node,
        Edge,
        Image,
    }

    #[napi(string_enum)]
    #[derive(Serialize, Deserialize, PartialEq)]
    pub enum NodeTypeCategory {
        Other,
        Sensor,
        Logic,
        Energy,
        Material,
        Aiuo,
        DCSPoint,
        GraphicEle,
        ScriptUO,
    }

    impl NodeTypeCategory {
        pub fn from_node(node: NodeType) -> Self {
            match node {
                NodeType::Material => Self::Material,
                NodeType::Energy => Self::Energy,
                NodeType::Aiuo => Self::Aiuo,
                NodeType::PositionInformation => Self::DCSPoint,
                NodeType::ScriptUO => Self::ScriptUO,
                NodeType::Signal
                | NodeType::ByPass
                | NodeType::Vote
                | NodeType::Rs
                | NodeType::Not
                | NodeType::Delay
                | NodeType::Pulse
                | NodeType::AndOr
                | NodeType::ScriptLogic
                | NodeType::CustomCustomerUO
                | NodeType::CustomSpecialUO
                | NodeType::CustomUserUO => Self::Logic,
                NodeType::TSensor
                | NodeType::PSensor
                | NodeType::LSensor
                | NodeType::CompSensor
                | NodeType::FSensor
                | NodeType::NvfSensor
                | NodeType::VfSensor
                | NodeType::DSensor => Self::Sensor,
                NodeType::Rect | NodeType::Ellipse | NodeType::Path | NodeType::Text => {
                    Self::GraphicEle
                }
                _ => Self::Other,
            }
        }
    }

    // 补充一个判断方法
    impl NodeType {
        /// 判断是否是逻辑模块
        pub fn is_logic_node_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                let category = NodeTypeCategory::from_node(node);
                // 如果分类是 Logic，或者是 ScriptUO (根据你之前的 TS 逻辑判断)
                if category == NodeTypeCategory::Logic {
                    return true;
                }
            }

            t.starts_with("customCustomerLogic")
                || t.starts_with("CustomCustomerLogic")
                || t.starts_with("customUserLogic")
                || t.starts_with("CustomUserLogic")
                || t.starts_with("customSysLogic")
                || t.starts_with("CustomSysLogic")
        }

        pub fn is_custom_customer_logic(t: &str) -> bool {
            t.starts_with("customCustomerLogic") || t.starts_with("CustomCustomerLogic")
        }

        pub fn is_custom_user_logic(t: &str) -> bool {
            t.starts_with("customUserLogic") || t.starts_with("customUserLogic")
        }

        pub fn is_custom_sys_logic(t: &str) -> bool {
            t.starts_with("customSysLogic") || t.starts_with("customSysLogic")
        }
        /// 判断是否是传感器模块
        pub fn is_sensor_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::Sensor;
            }
            false
        }
        /// 判断是否是ai模块
        pub fn is_aiuo_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::Aiuo;
            }
            false
        }
        /// 判断是否是脚本uo模块
        pub fn is_script_uo_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::ScriptUO;
            }
            false
        }

        /// 判断是否是其他单元模块
        pub fn is_other_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::Other;
            }
            false
        }

        /// 判断是否是物流模块
        pub fn is_material_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::Material;
            }
            false
        }

        /// 判断是否是能流模块
        pub fn is_enery_from_str(t: &str) -> bool {
            if let Ok(node) = NodeType::from_str(t) {
                return NodeTypeCategory::from_node(node) == NodeTypeCategory::Energy;
            }
            false
        }
    }
}
