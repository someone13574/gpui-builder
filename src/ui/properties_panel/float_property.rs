use gpui::*;

use crate::component::element::ComponentElement;
use crate::component::element_property::ElementProperty;
use crate::ui::text_entry::{TextEntry, TextModel};

pub struct FloatProperty {
    element: ComponentElement,
    property_name: String,
    text_entry: View<TextEntry>,
}

impl FloatProperty {
    pub fn new<V: 'static>(
        property: Model<(String, ElementProperty)>,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let (property_name, property_value) = property.read(cx).clone();
            let text_model = TextModel::new(f32::from(property_value).to_string(), cx);
            Self::observe_entry(&text_model, cx);

            let text_entry = TextEntry::new(text_model, |char| char.is_ascii_hexdigit(), cx);

            Self {
                element,
                property_name,
                text_entry,
            }
        })
    }

    fn observe_entry(text_model: &Model<TextModel>, cx: &mut ViewContext<Self>) {
        cx.observe(text_model, |this, text_model, cx| {
            let text = &text_model.read(cx).text;
            if let Ok(value) = text.parse::<f32>() {
                this.element
                    .set_property(&this.property_name, value.into(), cx);
            }
        })
        .detach();
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
