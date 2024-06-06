use gpui::{rgba, AppContext, Context, Model};
use indexmap::IndexMap;
use uuid::Uuid;

use super::ComponentElement;
use crate::component::element_property::{
    enum_property::cursor::cursor_property, insert_property, ElementProperty,
};

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

        insert_property("flex", false.into(), &mut properties, cx);
        insert_property("visible", true.into(), &mut properties, cx);
        insert_property("overflow_x_hidden", false.into(), &mut properties, cx);
        insert_property("overflow_y_hidden", false.into(), &mut properties, cx);
        insert_property("margin_left", 0.0.into(), &mut properties, cx);
        insert_property("margin_right", 0.0.into(), &mut properties, cx);
        insert_property("margin_top", 0.0.into(), &mut properties, cx);
        insert_property("margin_bottom", 0.0.into(), &mut properties, cx);
        insert_property("padding_left", 0.0.into(), &mut properties, cx);
        insert_property("padding_right", 0.0.into(), &mut properties, cx);
        insert_property("padding_top", 0.0.into(), &mut properties, cx);
        insert_property("padding_bottom", 0.0.into(), &mut properties, cx);
        insert_property("border_left", 0.0.into(), &mut properties, cx);
        insert_property("border_right", 0.0.into(), &mut properties, cx);
        insert_property("border_top", 0.0.into(), &mut properties, cx);
        insert_property("border_bottom", 0.0.into(), &mut properties, cx);
        insert_property("gap_x", 0.0.into(), &mut properties, cx);
        insert_property("gap_y", 0.0.into(), &mut properties, cx);
        insert_property("background", rgba(0x00000000).into(), &mut properties, cx);
        insert_property("border_color", rgba(0x00000000).into(), &mut properties, cx);
        insert_property("radius_top_left", 0.0.into(), &mut properties, cx);
        insert_property("radius_top_right", 0.0.into(), &mut properties, cx);
        insert_property("radius_bottom_left", 0.0.into(), &mut properties, cx);
        insert_property("radius_bottom_right", 0.0.into(), &mut properties, cx);
        insert_property(
            "cursor_style",
            cursor_property().into(),
            &mut properties,
            cx,
        );

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
