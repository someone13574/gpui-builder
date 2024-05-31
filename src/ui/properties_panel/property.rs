use gpui::*;

use super::float_property::FloatProperty;
use crate::component::element::property::{ElementProperty, ElementPropertyType};
use crate::component::element::ComponentElement;

#[derive(IntoElement, Clone)]
pub enum Property {
    Float(View<FloatProperty>),
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
            ElementPropertyType::_String => todo!(),
        }
    }
}

impl RenderOnce for Property {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Float(element) => div().child(element),
        }
    }
}
