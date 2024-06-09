use gpui::*;

use crate::component::element_property::ElementProperty;
use crate::ui::text_entry::{TextEntry, TextModel};

pub struct TextProperty {
    property: Model<ElementProperty>,
    property_name: String,
    text_entry: View<TextEntry>,
}

impl TextProperty {
    pub fn new<V: 'static>(
        property: Model<ElementProperty>,
        property_name: String,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let property_value = property.read(cx).clone().into();
            let text_model = TextModel::new(property_value, cx);
            Self::observe_entry(&text_model, cx);

            let text_entry = TextEntry::new(text_model, |_| true, cx);

            Self {
                property,
                property_name,
                text_entry,
            }
        })
    }

    fn observe_entry(text_model: &Model<TextModel>, cx: &mut ViewContext<Self>) {
        cx.observe(text_model, |this, text_model, cx| {
            let text = text_model.read(cx).text.clone();
            this.property.update(cx, |property, cx| {
                *property = text.into();
                cx.notify();
            });
        })
        .detach();
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
