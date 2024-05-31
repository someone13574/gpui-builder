use gpui::{AppContext, Context};
use uuid::Uuid;

use super::property::ElementProperty;
use super::ComponentElement;

#[derive(Clone)]
pub struct DivElement {
    pub children: Vec<ComponentElement>,
    pub id: Uuid,
    pub properties: Vec<ElementProperty>,
}

impl DivElement {
    pub fn new() -> Self {
        let rounded_property = ElementProperty::new_float("rounded", Some(0.0), None, 0.0);

        Self {
            children: Vec::new(),
            id: Uuid::new_v4(),
            properties: vec![rounded_property],
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
