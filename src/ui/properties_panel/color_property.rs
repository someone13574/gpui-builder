use gpui::*;

use crate::component::element::property::{format_rgba, parse_rgba};
use crate::component::element::ComponentElement;
use crate::ui::text_entry::{TextEntry, TextModel};

pub struct ColorProperty {
    property_name: String,
    element: ComponentElement,
    text_entry: View<TextEntry>,
}

impl ColorProperty {
    pub fn new<V: 'static>(
        property_name: String,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            element.observe_notify(cx);

            let model =
                TextModel::new(format_rgba(element.property(&property_name, cx).into()), cx);
            cx.observe(&model, |this: &mut Self, text, cx| {
                let text = &text.read(cx).text;
                let rgba = if let Some(rgba) = parse_rgba(text) {
                    rgba
                } else {
                    rgba(0xff00ffff)
                };

                this.element
                    .set_property(this.property_name.clone(), rgba.into(), cx);
            })
            .detach();

            let text_entry = TextEntry::new(model, |char| char.is_ascii_hexdigit(), cx);

            Self {
                property_name,
                element,
                text_entry,
            }
        })
    }
}

impl Render for ColorProperty {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(format!("{}: ", self.property_name))
            .child(self.text_entry.clone())
    }
}
