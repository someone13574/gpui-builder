use std::collections::HashMap;

use gpui::{AppContext, Context};
use uuid::Uuid;

use super::property::ElementProperty;
use super::ComponentElement;

#[derive(Clone)]
pub struct TextElement {
    pub id: Uuid,
    pub properties: HashMap<String, ElementProperty>,
}

impl TextElement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(text: &str, cx: &mut AppContext) -> ComponentElement {
        let mut properties = HashMap::new();
        properties.insert("text".to_string(), text.to_string().into());

        ComponentElement::Text(cx.new_model(|_| Self {
            id: Uuid::new_v4(),
            properties,
        }))
    }
}
