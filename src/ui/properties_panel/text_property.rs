use gpui::*;

use crate::component::element::property::ElementProperty;
use crate::component::element::ComponentElement;
use crate::ui::text_entry::{TextEntry, TextModel};

pub struct TextProperty {
    property_name: String,
    element: ComponentElement,
    text_entry: View<TextEntry>,
}

impl TextProperty {
    pub fn new<V: 'static>(
        property_name: String,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            element.observe_notify(cx);

            let model = TextModel::new(String::from(element.property(&property_name, cx)), cx);
            cx.observe(&model, |this: &mut Self, text, cx| {
                let text = text.read(cx).text.clone();
                this.element.set_property(
                    this.property_name.clone(),
                    ElementProperty::Text(text),
                    cx,
                );
            })
            .detach();

            let text_entry = TextEntry::new(model, Box::new(|_| true), cx);
            Self {
                property_name,
                element,
                text_entry,
            }
        })
    }
}

impl Render for TextProperty {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .flex()
            .flex_row()
            .child(format!("{}: ", self.property_name))
            .child(self.text_entry.clone())
    }
}
