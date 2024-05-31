use gpui::*;

use crate::component::element::{property, ComponentElement};
use crate::ui::text_entry::{TextEntry, TextModel};

pub struct FloatProperty {
    property_name: String,
    element: ComponentElement,
    text_entry: View<TextEntry>,
}

impl FloatProperty {
    pub fn new<V: 'static>(
        property_name: String,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            element.observe_notify(cx);

            let model = TextModel::new(
                format!(
                    "{}",
                    property::FloatProperty::from(element.property(&property_name, cx).content)
                        .value
                ),
                cx,
            );
            cx.observe(&model, |this: &mut Self, text, cx| {
                let text = &text.read(cx).text;
                let value = if let Ok(value) = text.parse::<f32>() {
                    value
                } else {
                    0.0
                };

                let mut current = property::FloatProperty::from(
                    this.element.property(&this.property_name, cx).content,
                );
                current.value = value;
                this.element.set_property(
                    &this.property_name,
                    property::ElementPropertyType::Float(current),
                    cx,
                );
            })
            .detach();

            let text_entry = TextEntry::new(
                model,
                Box::new(|key| key.chars().next().unwrap().is_numeric()),
                cx,
            );

            Self {
                property_name,
                element,
                text_entry,
            }
        })
    }
}

impl Render for FloatProperty {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(format!("{}: ", self.property_name))
            .child(self.text_entry.clone())
    }
}
