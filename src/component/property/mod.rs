pub mod bool_prop;
pub mod color_prop;
pub mod enum_prop;
pub mod float_prop;
pub mod text_prop;

use enum_prop::EnumProperty;
use gpui::{AppContext, Context, Model, Rgba};
use indexmap::IndexMap;

#[derive(Clone, Debug)]
pub enum ComponentProperty {
    Bool(bool),
    Color(Rgba),
    Enum(EnumProperty),
    Float(f32),
    Text(String),
}

impl ComponentProperty {
    pub fn to_model(&self, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| self.clone())
    }
}

pub fn to_model_with_default<T: Into<ComponentProperty>>(
    property: T,
    cx: &mut AppContext,
) -> (ComponentProperty, Model<ComponentProperty>) {
    let property = property.into();
    (property.clone(), property.to_model(cx))
}

pub fn read_properties(
    properties: &IndexMap<String, (ComponentProperty, Model<ComponentProperty>)>,
    cx: &mut AppContext,
) -> IndexMap<String, ComponentProperty> {
    properties
        .iter()
        .map(|(key, (_, value))| (key.to_string(), value.read(cx).clone()))
        .collect()
}
