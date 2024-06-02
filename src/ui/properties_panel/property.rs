use gpui::*;

use super::color_property::ColorProperty;
use super::float_property::FloatProperty;
use super::text_property::TextProperty;
use crate::component::element::ComponentElement;
use crate::component::element_property::ElementProperty;

#[derive(IntoElement, Clone)]
pub enum Property {
    Float(View<FloatProperty>),
    Text(View<TextProperty>),
    Color(View<ColorProperty>),
}

impl Property {
    pub fn new<V: 'static>(
        property: Model<(String, ElementProperty)>,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match property.read(cx).1 {
            ElementProperty::Float(_) => {
                Self::Float(FloatProperty::new(property, element.clone(), cx))
            }
            ElementProperty::Text(_) => {
                Self::Text(TextProperty::new(property, element.clone(), cx))
            }
            ElementProperty::Color(_) => {
                Self::Color(ColorProperty::new(property, element.clone(), cx))
            }
        }
    }
}

impl RenderOnce for Property {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Float(element) => div().child(element),
            Self::Text(element) => div().child(element),
            Self::Color(element) => div().child(element),
        }
    }
}
