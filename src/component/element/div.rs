use std::collections::HashMap;

use gpui::{AppContext, Context};
use uuid::Uuid;

use super::property::{ElementProperty, FloatProperty};
use super::ComponentElement;

#[derive(Clone)]
pub struct DivElement {
    pub id: Uuid,
    pub children: Vec<ComponentElement>,
    pub properties: HashMap<String, ElementProperty>,
}

impl DivElement {
    pub fn new() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            "rounded".to_string(),
            FloatProperty {
                min: Some(0.0),
                max: None,
                value: 0.0,
            }
            .into(),
        );

        Self {
            id: Uuid::new_v4(),
            children: Vec::new(),
            properties,
        }
    }

    pub fn child(mut self, child: ComponentElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self, cx: &mut AppContext) -> ComponentElement {
        ComponentElement::Div(cx.new_model(|_| self))
    }
}
