use div::DivElement;
use serde::Deserialize;
use text::TextElement;
use uuid::Uuid;

pub mod div;
pub mod text;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentElement {
    Div(DivElement),
    Text(TextElement),
}

impl ComponentElement {
    pub fn id(&self) -> Option<Uuid> {
        match self {
            ComponentElement::Div(element) => element.id,
            ComponentElement::Text(element) => element.id,
        }
    }

    pub fn assign_id_recursive(&mut self) {
        match self {
            ComponentElement::Div(element) => element.assign_id_recursive(),
            ComponentElement::Text(element) => element.assign_id_recursive(),
        }
    }
}

impl From<ComponentElement> for String {
    fn from(value: ComponentElement) -> Self {
        match value {
            ComponentElement::Div(_) => "div:".to_string(),
            ComponentElement::Text(text) => format!("\"{}\"", text.text),
        }
    }
}
