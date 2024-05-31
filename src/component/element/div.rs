use super::ComponentElement;
use gpui::{AppContext, Context};
use uuid::Uuid;

#[derive(Clone)]
pub struct DivElement {
    pub children: Vec<ComponentElement>,
    pub id: Uuid,
}

impl DivElement {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            id: Uuid::new_v4(),
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
