use gpui::*;
use prelude::FluentBuilder;

use crate::appearance::sizes;

pub type OnClickFn = dyn Fn(&ClickEvent, &mut WindowContext);

pub struct Button {
    label: String,
    hovered: bool,
    on_click: Box<OnClickFn>,
}

impl Button {
    pub fn new(
        label: &str,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
        cx: &mut WindowContext,
    ) -> View<Self> {
        cx.new_view(|_cx| Self {
            label: label.to_string(),
            hovered: false,
            on_click: Box::new(on_click),
        })
    }
}

impl Render for Button {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .px_1()
            .rounded(px(4.0))
            .text_size(*sizes::TEXT_SIZE_SMALL)
            .child(self.label.clone())
            .id("button")
            .on_click(cx.listener(|this, event, cx| {
                cx.stop_propagation();
                (this.on_click)(event, cx)
            }))
            .on_hover(cx.listener(|this, hovered, cx| {
                this.hovered = *hovered;
                cx.notify();
            }))
            .when(self.hovered, |this_div| this_div.bg(rgba(0xffffff04)))
    }
}
