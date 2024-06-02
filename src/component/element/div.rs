use gpui::{rgb, AppContext, Context, Model};
use indexmap::IndexMap;
use uuid::Uuid;

use super::ComponentElement;
use crate::component::element_property::{insert_property, ElementProperty};

#[derive(Clone)]
pub struct DivElement {
    pub id: Uuid,
    pub children: Model<Vec<ComponentElement>>,
    pub properties: IndexMap<String, Model<(String, ElementProperty)>>,
}

impl DivElement {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            children: cx.new_model(|_| Vec::new()),
            properties: Self::default_properties(cx),
        }
    }

    fn default_properties(
        cx: &mut AppContext,
    ) -> IndexMap<String, Model<(String, ElementProperty)>> {
        let mut properties = IndexMap::new();

        insert_property("rounding", 0.0.into(), &mut properties, cx);
        insert_property("padding", 16.0.into(), &mut properties, cx);
        insert_property("bg", rgb(0x808080).into(), &mut properties, cx);

        properties
    }
}

impl From<ComponentElement> for DivElement {
    fn from(value: ComponentElement) -> Self {
        if let ComponentElement::Div(element) = value {
            element
        } else {
            unreachable!()
        }
    }
}
