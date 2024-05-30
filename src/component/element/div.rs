use super::ComponentElement;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DivElement {
    #[serde(rename = "$value", default)]
    pub children: Vec<ComponentElement>,
}
