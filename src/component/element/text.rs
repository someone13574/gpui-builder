use gpui::{AppContext, Context};
use uuid::Uuid;

use super::property::ElementProperty;
use super::ComponentElement;

#[derive(Clone)]
pub struct TextElement {
    pub id: Uuid,
    pub properties: Vec<ElementProperty>,
}

impl TextElement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(text: &str, cx: &mut AppContext) -> ComponentElement {
        let text_property = ElementProperty::new_text("text", text);

        ComponentElement::Text(cx.new_model(|_| Self {
            id: Uuid::new_v4(),
            properties: vec![text_property],
        }))
    }
}
