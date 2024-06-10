use gpui::*;

use super::bool_property::BoolProperty;
use super::color_property::ColorProperty;
use super::enum_property::EnumProperty;
use super::float_property::FloatProperty;
use super::text_property::TextProperty;
use crate::component::property::ComponentProperty;

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
        property: Model<ComponentProperty>,
        property_name: String,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match property.read(cx) {
            ComponentProperty::Float(_) => {
                Self::Float(FloatProperty::new(property, property_name, cx))
            }
            ComponentProperty::Bool(_) => {
                Self::Bool(BoolProperty::new(property, property_name, cx))
            }
            ComponentProperty::Text(_) => {
                Self::Text(TextProperty::new(property, property_name, cx))
            }
            ComponentProperty::Color(_) => {
                Self::Color(ColorProperty::new(property, property_name, cx))
            }
            ComponentProperty::Enum(_) => {
                Self::Enum(EnumProperty::new(property, property_name, cx))
            }
        }
    }
}

impl RenderOnce for Property {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            Self::Float(component) => div().child(component),
            Self::Bool(component) => div().child(component),
            Self::Text(component) => div().child(component),
            Self::Color(component) => div().child(component),
            Self::Enum(component) => div().child(component),
        }
    }
}
