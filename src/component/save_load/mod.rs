pub mod from_serde;
pub mod rust_generator;
pub mod to_serde;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SerdeComponent {
    Div(SerdeDiv),
    Text(SerdeText),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeDiv {
    id: Uuid,
    properties: Vec<SerdeProperty>,
    children: Vec<SerdeComponent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeText {
    id: Uuid,
    properties: Vec<SerdeProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeProperty {
    pub key: String,
    pub value: String,
    pub default: String,
    pub property_type: SerdePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SerdePropertyType {
    Bool,
    Color,
    Enum,
    Float,
    Text,
}
