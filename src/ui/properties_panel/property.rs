use gpui::*;

use super::float_property::FloatProperty;
use super::text_property::TextProperty;
use crate::component::element::property::ElementProperty;
use crate::component::element::ComponentElement;

#[derive(IntoElement, Clone)]
pub enum Property {
    Float(View<FloatProperty>),
    Text(View<TextProperty>),
}

impl Property {
    pub fn new<V: 'static>(
        property_name: &str,
        property: &ElementProperty,
        element: &ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match &property {
            ElementProperty::Float(_) => Self::Float(FloatProperty::new(
                property_name.to_string(),
                element.clone(),
                cx,
            )),
            ElementProperty::Text(_) => Self::Text(TextProperty::new(
                property_name.to_string(),
                element.clone(),
                cx,
            )),
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
