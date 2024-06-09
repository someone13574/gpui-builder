use gpui::{rgba, AppContext, Context, Model};
use indexmap::IndexMap;
use uuid::Uuid;

use super::property::enum_prop::cursor::cursor_enum_property;
use super::property::enum_prop::display::display_enum_property;
use super::property::enum_prop::overflow::overflow_enum_property;
use super::property::{to_model, ComponentProperty};
use super::Component;

#[derive(Clone, Debug)]
pub struct DivComponent {
    pub id: Uuid,
    pub parent: Option<Box<Component>>,

    pub children: Model<Vec<Component>>,
    pub properties: IndexMap<String, Model<ComponentProperty>>,
}

impl DivComponent {
    pub fn new(cx: &mut AppContext) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            children: cx.new_model(|_| Vec::new()),
            properties: Self::default_properties(cx),
        }
    }

    pub fn add_child(&self, child: Component, cx: &mut AppContext) {
        let child = child.with_parent(self.clone().into());
        self.children.update(cx, |children, cx| {
            children.push(child);
            cx.notify()
        });
    }

    fn default_properties(cx: &mut AppContext) -> IndexMap<String, Model<ComponentProperty>> {
        let mut properties = IndexMap::new();

        properties.insert("display".to_string(), to_model(display_enum_property(), cx));
        properties.insert("visible".to_string(), to_model(true, cx));
        properties.insert(
            "overflow_x".to_string(),
            to_model(overflow_enum_property(), cx),
        );
        properties.insert(
            "overflow_y".to_string(),
            to_model(overflow_enum_property(), cx),
        );
        properties.insert("margin_left".to_string(), to_model(0.0, cx));
        properties.insert("margin_right".to_string(), to_model(0.0, cx));
        properties.insert("margin_top".to_string(), to_model(0.0, cx));
        properties.insert("margin_bottom".to_string(), to_model(0.0, cx));
        properties.insert("padding_left".to_string(), to_model(0.0, cx));
        properties.insert("padding_right".to_string(), to_model(0.0, cx));
        properties.insert("padding_top".to_string(), to_model(0.0, cx));
        properties.insert("padding_bottom".to_string(), to_model(0.0, cx));
        properties.insert("border_left".to_string(), to_model(0.0, cx));
        properties.insert("border_right".to_string(), to_model(0.0, cx));
        properties.insert("border_top".to_string(), to_model(0.0, cx));
        properties.insert("border_bottom".to_string(), to_model(0.0, cx));
        properties.insert("gap_x".to_string(), to_model(0.0, cx));
        properties.insert("gap_y".to_string(), to_model(0.0, cx));
        properties.insert("background".to_string(), to_model(rgba(0), cx));
        properties.insert("border_color".to_string(), to_model(rgba(0), cx));
        properties.insert("radius_top_left".to_string(), to_model(0.0, cx));
        properties.insert("radius_top_right".to_string(), to_model(0.0, cx));
        properties.insert("radius_bottom_right".to_string(), to_model(0.0, cx));
        properties.insert("radius_bottom_left".to_string(), to_model(0.0, cx));
        properties.insert(
            "cursor_style".to_string(),
            to_model(cursor_enum_property(), cx),
        );

        properties
    }
}

impl From<Component> for DivComponent {
    fn from(value: Component) -> Self {
        if let Component::Div(value) = value {
            value
        } else {
            unreachable!("Couldn't downcast {value:?} into a DivComponent")
        }
    }
}

impl From<DivComponent> for Component {
    fn from(value: DivComponent) -> Self {
        Component::Div(value)
    }
}
