use gpui::*;

use crate::component::element::{property, ComponentElement};

pub struct FloatProperty {
    property_name: String,
    element: ComponentElement,
}

impl FloatProperty {
    pub fn new<V: 'static>(
        property_name: String,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            element.observe_notify(cx);

            Self {
                property_name,
                element,
            }
        })
    }
}

impl Render for FloatProperty {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let property: property::FloatProperty = self
            .element
            .property(&self.property_name, cx)
            .content
            .into();

        div()
            .flex()
            .flex_row()
            .child(self.property_name.clone())
            .child(format!(": {}", property.value))
    }
}
