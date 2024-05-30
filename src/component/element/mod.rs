use div::DivElement;
use serde::Deserialize;
use text::TextElement;

pub mod div;
pub mod text;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ComponentElement {
    Div(DivElement),
    Text(TextElement),
}

impl From<ComponentElement> for String {
    fn from(value: ComponentElement) -> Self {
        match value {
            ComponentElement::Div(_) => "div:".to_string(),
            ComponentElement::Text(text) => format!("\"{}\"", text.0),
        }
    }
}
