use gpui::*;

use super::float_property::FloatProperty;
use super::text_property::TextProperty;
use crate::component::element::property::{ElementProperty, ElementPropertyType};
use crate::component::element::ComponentElement;

#[derive(IntoElement, Clone)]
pub enum Property {
    Float(View<FloatProperty>),
    Text(View<TextProperty>),
}

impl Property {
    pub fn new<V: 'static>(
        property: &ElementProperty,
        element: &ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> Self {
        let name = &property.name;
        match &property.content {
            ElementPropertyType::Float(_) => {
                Self::Float(FloatProperty::new(name.clone(), element.clone(), cx))
            }
            ElementPropertyType::Text(_) => {
                Self::Text(TextProperty::new(name.clone(), element.clone(), cx))
            }
        }
    }
}

impl RenderOnce for Property {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Float(element) => div().child(element),
            Self::Text(element) => div().child(element),
        }
    }
}
