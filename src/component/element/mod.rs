use div::DivElement;
use gpui::AppContext;
use text::TextElement;
use uuid::Uuid;

use super::element_property::ElementProperty;

pub mod div;
pub mod text;

#[derive(Clone)]
pub enum ComponentElement {
    Div(DivElement),
    Text(TextElement),
}

impl ComponentElement {
    pub fn id(&self) -> Uuid {
        match self {
            ComponentElement::Div(element) => element.id,
            ComponentElement::Text(element) => element.id,
        }
    }

    pub fn set_property(&self, key: &str, value: ElementProperty, cx: &mut AppContext) {
        match self {
            ComponentElement::Div(element) => {
                let model = element.properties.get(key).unwrap();
                model.update(cx, |(_, property), cx| {
                    *property = value;
                    cx.notify();
                })
            }
            ComponentElement::Text(element) => {
                let model = element.properties.get(key).unwrap();
                model.update(cx, |(_, property), cx| {
                    *property = value;
                    cx.notify();
                })
            }
        }
    }

    pub fn find_parent_recursive(&self, id: Uuid, cx: &mut AppContext) -> Option<Self> {
        let children = match self {
            ComponentElement::Div(element) => element.children.read(cx).clone(),
            ComponentElement::Text(_) => return None,
        };

        for child in children {
            if child.id() == id {
                return Some(self.clone());
            }

            if let Some(parent_id) = child.find_parent_recursive(id, cx) {
                return Some(parent_id);
            }
        }

        None
    }
}

impl From<DivElement> for ComponentElement {
    fn from(value: DivElement) -> Self {
        Self::Div(value)
    }
}

impl From<TextElement> for ComponentElement {
    fn from(value: TextElement) -> Self {
        Self::Text(value)
    }
}
