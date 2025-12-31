use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(namespace = "modelType")]
pub mod model_type {
    use super::*;

    /// 传感器节点名称
    #[napi(string_enum)]
    pub enum ModelSensorNodeName {
        #[napi(value = "温度传感器")]
        TSensor,
        #[napi(value = "压力传感器")]
        PSensor,
        #[napi(value = "液位传感器")]
        LSensor,
        #[napi(value = "组分传感器")]
        CompSensor,
        #[napi(value = "流量传感器")]
        FSensor,
        #[napi(value = "标准体积流量传感器")]
        NvfSensor,
        #[napi(value = "体积流量传感器")]
        VfSensor,
        #[napi(value = "密度传感器")]
        DSensor,
    }

    /// 自定义节点名称
    #[napi(string_enum)]
    pub enum CustomNodeName {
        #[napi(value = "脚本逻辑")]
        ScriptLogic,
        #[napi(value = "脚本模块")]
        ScriptUO,
        #[napi(value = "自定义算法")]
        CustomUserUO,
        #[napi(value = "电加热")]
        CustomUserLogic0,
        #[napi(value = "防喘振控制器")]
        CustomUserLogic1,
        #[napi(value = "ITCC压缩机升速")]
        CustomUserLogic2,
        #[napi(value = "轴系监测")]
        CustomUserLogic3,
    }

    /// 节点类型核心枚举
    #[napi(string_enum)]
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
        CustomVueNode,
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
        CustomUserLogic0,
        CustomUserLogic1,
        CustomUserLogic2,
        CustomUserLogic3,
    }

    /// 自定义模块前缀
    #[napi(string_enum)]
    pub enum NodeTypeCustom {
        CustomCustomerUO,
        CustomUserUO,
        CustomSpecialUO,
        CustomSysUO,
        CustomUserLogic,
        CustomCustomerLogic,
        CustomSysLogic,
    }

    /// 图形/连线分类
    #[napi(string_enum)]
    #[derive(Serialize, Deserialize)]
    pub enum GraphType {
        Node,
        Edge,
        Image,
    }

    // --- 结构体与 Interface 转换 ---

    // #[napi(object)]
    // #[derive(Serialize, Deserialize)]
    // pub struct ModelFlowSheetUpdateDTO {
    //     pub id: String,
    //     pub name: Option<String>,
    //     pub description: Option<String>,
    //     // 其他字段根据 ModelGraphicPageEntity 补充
    // }

    // #[napi(object)]
    // #[derive(Serialize, Deserialize)]
    // pub struct GraphListData {
    //     pub type_name: GraphType, // 使用 r# 避开 Rust 关键字
    //     pub data: String,         // 建议存为 JSON 字符串
    // }

    // #[napi(object)]
    // #[derive(Serialize, Deserialize)]
    // pub struct ModelFlowSheetGraphListInsertDTO {
    //     pub graphic_page_id: String,
    //     pub graph_list: Vec<GraphListData>,
    // }

    // #[napi(object)]
    // #[derive(Serialize, Deserialize)]
    // pub struct ModelGraphCheckListDTO {
    //     pub id: String,
    //     pub key: String, // "material" | "energy"
    //     pub model_id: String,
    //     pub r#type: String, // "in" | "out" | "energyStreamIn"
    // }
}
