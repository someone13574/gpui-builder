pub mod conversion;

use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SerdeComponent {
    Div(SerdeDiv),
    Text(SerdeText),
}

#[derive(Serialize, Debug)]
pub struct SerdeDiv {
    id: Uuid,
    properties: Vec<SerdeProperty>,
    children: Vec<SerdeComponent>,
}

#[derive(Serialize, Debug)]
pub struct SerdeText {
    id: Uuid,
    properties: Vec<SerdeProperty>,
}

#[derive(Serialize, Debug)]
pub struct SerdeProperty {
    key: String,
    value: String,
    property_type: SerdePropertyType,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SerdePropertyType {
    Bool,
    Color,
    Enum,
    Float,
    Text,
}
