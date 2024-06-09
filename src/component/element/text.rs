use gpui::{AppContext, Model};
use indexmap::IndexMap;
use uuid::Uuid;

use crate::component::element_property::{insert_property, ElementProperty};

#[derive(Clone)]
pub struct TextElement {
    pub id: Uuid,
    pub properties: IndexMap<String, Model<ElementProperty>>,
}

impl TextElement {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            properties: Self::default_properties(cx),
        }
    }

    fn default_properties(cx: &mut AppContext) -> IndexMap<String, Model<ElementProperty>> {
        let mut properties = IndexMap::new();
        insert_property("text", "New text".to_string().into(), &mut properties, cx);
        properties
    }
}
