use crate::{appearance::colors, component::element::ComponentElement};
use gpui::*;

pub struct TreeviewItem {
    element: ComponentElement,
    indent: u32,
}

impl TreeviewItem {
    pub fn new<V: 'static>(
        element: ComponentElement,
        indent: u32,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|_| Self { element, indent })
    }
}

impl Render for TreeviewItem {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .children((0..self.indent).map(|_| {
                div()
                    .min_w(px(32.0))
                    .border_l_1()
                    .border_color(*colors::BORDER)
            }))
            .child(String::from(self.element.clone()))
    }
}
