use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "component")]
pub struct Component {
    #[serde(rename = "$value")]
    pub element: Element,
}

impl Component {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, serde_xml_rs::Error> {
        let src = fs::read_to_string(path).unwrap();
        serde_xml_rs::from_str(&src)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Element {
    Div(DivElement),
    Text(String),
}

impl From<Element> for String {
    fn from(value: Element) -> Self {
        match value {
            Element::Div(_) => "div:".to_string(),
            Element::Text(text) => format!("\"{text}\""),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DivElement {
    #[serde(rename = "$value", default)]
    pub children: Vec<Element>,
}
