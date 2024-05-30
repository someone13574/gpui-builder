use crate::{
    appearance::{colors, sizes},
    component::element::ComponentElement,
};
use gpui::*;
use prelude::FluentBuilder;

pub struct TreeviewItem {
    element: ComponentElement,
    indent: u32,

    hover: bool,
}

impl TreeviewItem {
    pub fn new<V: 'static>(
        element: ComponentElement,
        indent: u32,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|_| Self {
            element,
            indent,
            hover: false,
        })
    }
}

impl Render for TreeviewItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .px_2()
            .children((0..self.indent).map(|_| {
                div()
                    .min_w(*sizes::TREEVIEW_INDENT)
                    .border_l_1()
                    .border_color(*colors::BORDER)
            }))
            .child(String::from(self.element.clone()))
            .id("treeview-item")
            .on_mouse_up(MouseButton::Left, |_, _| {})
            .on_hover(cx.listener(|this, hover, cx| {
                this.hover = *hover;
                cx.notify();
            }))
            .when(self.hover, |this| this.bg(*colors::LIST_ITEM_HOVER))
    }
}
