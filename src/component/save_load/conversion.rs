use gpui::AppContext;

use super::{SerdeComponent, SerdeDiv, SerdeProperty, SerdePropertyType, SerdeText};
use crate::component::div::DivComponent;
use crate::component::property::color_prop::format_rgba;
use crate::component::property::ComponentProperty;
use crate::component::text::TextComponent;
use crate::component::Component;

pub trait ToSerde<T>: Sized {
    fn to_serde(&self, cx: &AppContext) -> T;
}

impl ToSerde<SerdeComponent> for Component {
    fn to_serde(&self, cx: &AppContext) -> SerdeComponent {
        match self {
            Component::Div(component) => SerdeComponent::Div(component.to_serde(cx)),
            Component::Text(component) => SerdeComponent::Text(component.to_serde(cx)),
        }
    }
}

impl ToSerde<SerdeProperty> for (String, ComponentProperty) {
    fn to_serde(&self, _cx: &AppContext) -> SerdeProperty {
        let (property_type, value) = match &self.1 {
            ComponentProperty::Bool(value) => (SerdePropertyType::Bool, value.to_string()),
            ComponentProperty::Color(value) => (SerdePropertyType::Color, format_rgba(*value)),
            ComponentProperty::Enum(value) => (SerdePropertyType::Enum, value.value.clone()),
            ComponentProperty::Float(value) => (SerdePropertyType::Float, value.to_string()),
            ComponentProperty::Text(value) => (SerdePropertyType::Text, value.clone()),
        };

        SerdeProperty {
            key: self.0.clone(),
            value,
            property_type,
        }
    }
}

impl ToSerde<SerdeDiv> for DivComponent {
    fn to_serde(&self, cx: &AppContext) -> SerdeDiv {
        let children = self.children.read(cx).clone();
        let children = children
            .into_iter()
            .map(|child| child.to_serde(cx))
            .collect();

        SerdeDiv {
            id: self.id,
            children,
            properties: self
                .properties
                .iter()
                .map(|(key, value)| (key.to_string(), value.read(cx).clone()).to_serde(cx))
                .collect(),
        }
    }
}

impl ToSerde<SerdeText> for TextComponent {
    fn to_serde(&self, cx: &AppContext) -> SerdeText {
        SerdeText {
            id: self.id,
            properties: self
                .properties
                .iter()
                .map(|(key, value)| (key.to_string(), value.read(cx).clone()).to_serde(cx))
                .collect(),
        }
    }
}
