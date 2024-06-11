use gpui::{AppContext, Model};
use indexmap::IndexMap;
use uuid::Uuid;

use super::property::{to_model_with_default, ComponentProperty};
use super::Component;

#[derive(Clone, Debug)]
pub struct TextComponent {
    pub id: Uuid,
    pub parent: Option<Box<Component>>,

    pub properties: IndexMap<String, (ComponentProperty, Model<ComponentProperty>)>,
}

impl TextComponent {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            properties: Self::default_properties(cx),
        }
    }

    fn default_properties(
        cx: &mut AppContext,
    ) -> IndexMap<String, (ComponentProperty, Model<ComponentProperty>)> {
        let mut properties = IndexMap::new();
        properties.insert(
            "text".to_string(),
            to_model_with_default("New text".to_string(), cx),
        );
        properties
    }
}

impl From<Component> for TextComponent {
    fn from(value: Component) -> Self {
        if let Component::Text(value) = value {
            value
        } else {
            unreachable!("Couldn't downcast {value:?} into a DivComponent")
        }
    }
}

impl From<TextComponent> for Component {
    fn from(value: TextComponent) -> Self {
        Component::Text(value)
    }
}
