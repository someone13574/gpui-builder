use gpui::{AppContext, Context};

use super::{SerdeComponent, SerdeDiv, SerdeProperty, SerdeText};
use crate::component::div::DivComponent;
use crate::component::property::color_prop::parse_rgba;
use crate::component::property::enum_prop::cursor::cursor_enum_property;
use crate::component::property::enum_prop::display::display_enum_property;
use crate::component::property::enum_prop::overflow::{
    overflow_enum_property, OverflowEnumDirection,
};
use crate::component::property::ComponentProperty;
use crate::component::save_load::SerdePropertyType;
use crate::component::text::TextComponent;
use crate::component::Component;

pub trait FromSerde<T>: Sized {
    fn from_serde(this: T, cx: &mut AppContext) -> Self;
}

impl FromSerde<SerdeComponent> for Component {
    fn from_serde(this: SerdeComponent, cx: &mut AppContext) -> Self {
        match this {
            SerdeComponent::Div(component) => DivComponent::from_serde(component, cx).into(),
            SerdeComponent::Text(component) => TextComponent::from_serde(component, cx).into(),
        }
    }
}

impl FromSerde<SerdeProperty> for (String, ComponentProperty) {
    fn from_serde(this: SerdeProperty, _cx: &mut AppContext) -> Self {
        let value = match this.property_type {
            SerdePropertyType::Bool => ComponentProperty::Bool(this.value.parse().unwrap()),
            SerdePropertyType::Color => ComponentProperty::Color(parse_rgba(&this.value).unwrap()),
            SerdePropertyType::Enum => ComponentProperty::Enum({
                let mut enum_prop = match this.key.as_str() {
                    "display" => display_enum_property(),
                    "overflow_x" => overflow_enum_property(OverflowEnumDirection::X),
                    "overflow_y" => overflow_enum_property(OverflowEnumDirection::Y),
                    "cursor_style" => cursor_enum_property(),
                    key => unreachable!("Unknown enum property {key}"),
                };
                enum_prop.value.clone_from(&this.value);
                enum_prop
            }),
            SerdePropertyType::Float => ComponentProperty::Float(this.value.parse().unwrap()),
            SerdePropertyType::Text => ComponentProperty::Text(this.value.clone()),
        };

        (this.key.clone(), value)
    }
}

impl FromSerde<SerdeDiv> for DivComponent {
    fn from_serde(this: SerdeDiv, cx: &mut AppContext) -> Self {
        let properties: Vec<(String, ComponentProperty)> = this
            .properties
            .into_iter()
            .map(|property| FromSerde::<SerdeProperty>::from_serde(property, cx))
            .collect();
        let properties = properties
            .into_iter()
            .map(|(key, property)| (key, cx.new_model(|_| property)))
            .collect();

        let this_component = DivComponent {
            id: this.id,
            parent: None,
            children: cx.new_model(|_| Vec::new()),
            properties,
        };

        this_component.children.update(cx, |children, cx| {
            let this_component = this_component.clone().into();
            *children = this
                .children
                .into_iter()
                .map(|child| Component::from_serde(child, cx).with_parent(&this_component))
                .collect();
        });
        this_component
    }
}

impl FromSerde<SerdeText> for TextComponent {
    fn from_serde(value: SerdeText, cx: &mut AppContext) -> Self {
        let properties: Vec<(String, ComponentProperty)> = value
            .properties
            .into_iter()
            .map(|property| FromSerde::<SerdeProperty>::from_serde(property, cx))
            .collect();
        let properties = properties
            .into_iter()
            .map(|(key, property)| (key, cx.new_model(|_| property)))
            .collect();

        TextComponent {
            id: value.id,
            parent: None,
            properties,
        }
    }
}
