use gpui::{AppContext, Context, Model, Rgba};
use indexmap::IndexMap;

pub mod color;

#[derive(Clone)]
pub enum ElementProperty {
    Float(f32),
    Bool(bool),
    Text(String),
    Color(Rgba),
}

impl ElementProperty {
    pub fn new_model(id: &str, property: Self, cx: &mut AppContext) -> Model<(String, Self)> {
        cx.new_model(|_| (id.to_string(), property))
    }
}

impl From<f32> for ElementProperty {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for ElementProperty {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for ElementProperty {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for ElementProperty {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Rgba> for ElementProperty {
    fn from(value: Rgba) -> Self {
        Self::Color(value)
    }
}

pub fn read_properties(
    properties: &IndexMap<String, Model<(String, ElementProperty)>>,
    cx: &mut AppContext,
) -> IndexMap<String, ElementProperty> {
    properties
        .iter()
        .map(|(key, value)| (key.to_string(), value.read(cx).1.clone()))
        .collect()
}

pub fn insert_property(
    id: &str,
    value: ElementProperty,
    properties: &mut IndexMap<String, Model<(String, ElementProperty)>>,
    cx: &mut AppContext,
) {
    properties.insert(id.to_string(), ElementProperty::new_model(id, value, cx));
}

impl From<ElementProperty> for f32 {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Float(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}

impl From<ElementProperty> for bool {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Bool(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}

impl From<ElementProperty> for String {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Text(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}

impl From<ElementProperty> for Rgba {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Color(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}
