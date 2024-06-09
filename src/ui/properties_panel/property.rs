use gpui::*;

use super::bool_property::BoolProperty;
use super::color_property::ColorProperty;
use super::enum_property::EnumProperty;
use super::float_property::FloatProperty;
use super::text_property::TextProperty;
use crate::component::element_property::ElementProperty;

#[derive(IntoElement, Clone)]
pub enum Property {
    Float(View<FloatProperty>),
    Bool(View<BoolProperty>),
    Text(View<TextProperty>),
    Color(View<ColorProperty>),
    Enum(View<EnumProperty>),
}

impl Property {
    pub fn new<V: 'static>(
        property: Model<ElementProperty>,
        property_name: String,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match property.read(cx) {
            ElementProperty::Float(_) => {
                Self::Float(FloatProperty::new(property, property_name, cx))
            }
            ElementProperty::Bool(_) => Self::Bool(BoolProperty::new(property, property_name, cx)),
            ElementProperty::Text(_) => Self::Text(TextProperty::new(property, property_name, cx)),
            ElementProperty::Color(_) => {
                Self::Color(ColorProperty::new(property, property_name, cx))
            }
            ElementProperty::Enum(_) => Self::Enum(EnumProperty::new(property, property_name, cx)),
        }
    }
}

impl RenderOnce for Property {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Float(element) => div().child(element),
            Self::Bool(element) => div().child(element),
            Self::Text(element) => div().child(element),
            Self::Color(element) => div().child(element),
            Self::Enum(element) => div().child(element),
        }
    }
}
