pub mod from_serde;
pub mod rust_export;
pub mod to_serde;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SerdeComponent {
    Div(SerdeDiv),
    Text(SerdeText),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerdeDiv {
    id: Uuid,
    properties: Vec<SerdeProperty>,
    children: Vec<SerdeComponent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerdeText {
    id: Uuid,
    properties: Vec<SerdeProperty>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SerdeProperty {
    pub key: String,
    pub value: String,
    pub property_type: SerdePropertyType,
    pub extra_data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SerdePropertyType {
    Bool,
    Color,
    Enum,
    Float,
    Text,
}
