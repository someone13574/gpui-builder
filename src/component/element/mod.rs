use div::DivElement;
use gpui::{AppContext, Context, Model};
use text::TextElement;
use uuid::Uuid;

pub mod div;
pub mod text;

#[derive(Clone)]
pub enum ComponentElement {
    Div(Model<DivElement>),
    Text(Model<TextElement>),
}

impl ComponentElement {
    pub fn id(&self, cx: &mut AppContext) -> Uuid {
        match &self {
            ComponentElement::Div(element) => cx.read_model(element, |element, _| element.id),
            ComponentElement::Text(element) => cx.read_model(element, |element, _| element.id),
        }
    }

    pub fn string(&self, cx: &mut AppContext) -> String {
        match &self {
            ComponentElement::Div(_) => "div:".to_string(),
            ComponentElement::Text(element) => {
                let element = cx.read_model(element, |element, _| element.clone());
                format!("\"{}\"", element.text)
            }
        }
    }
}
