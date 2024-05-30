use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "text")]
pub struct TextElement(pub String);
