pub mod graphics;
pub mod nodes;
pub mod textbox;
pub mod log;
pub mod editor;

pub mod file;
pub mod github;
pub mod cookies;

pub const NODE_CIRCLE_RADIUS: f32 = 12.0;
pub const MIN_ZOOM: f32 = 8.0;
pub const MAX_ZOOM: f32 = 50.0;
pub const ZOOM_DELTA: f32 = 2.0;
pub const LINE_DISTANCE: f32 = 1.2;

pub const TOKEN_KEY: &str = "github-token";

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq)]
pub enum NodeType {
    Function,
    Constant,
    Variable,
    Comment,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub enum PropertyType {
    Bool,
    Number,
    String,
    List,
    File,
    Flow,
    Custom(String),
}

impl ToString for PropertyType {
    fn to_string(&self) -> String {
        match self {
            PropertyType::Bool => {
                "Boolean".to_owned()
            },
            PropertyType::Number => {
                "Number".to_owned()
            },
            PropertyType::String => {
                "String".to_owned()
            },
            PropertyType::List => {
                "List".to_owned()
            },
            PropertyType::File => {
                "File".to_owned()
            },
            PropertyType::Flow => {
                "Flow".to_owned()
            },
            PropertyType::Custom(t) => {
                format!("Custom ({})", t)
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct NodeConnector {
    pub id: u64,
    pub connector_type: PropertyType,
    pub connected: Option<u64>,
    pub connected_idx: Option<u32>,
    pub name: String,
    pub selected: bool,
}

impl AsRef<NodeConnector> for NodeConnector {
    fn as_ref(&self) -> &NodeConnector {
        self
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub id: u64,
    pub name: String,
    pub position: [f32; 2],
    pub selected: bool,
    pub input_connectors: Vec<NodeConnector>,
    pub output_connectors: Vec<NodeConnector>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct List {
    pub list_type: PropertyType,
    pub list_items: Vec<String>,
}

fn get_connector_height(between_buffer: f32, element_height: f32, idx: u32) -> f32 {
    between_buffer * idx as f32 + element_height * idx as f32
}
