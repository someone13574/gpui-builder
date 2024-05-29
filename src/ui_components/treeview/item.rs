use gpui::*;
use prelude::FluentBuilder;

use crate::{appearance::colors, hierarchy::Element};

#[derive(IntoElement)]
pub struct TreeviewItem {
    pub element: Element,
    pub root: bool,
}

impl RenderOnce for TreeviewItem {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let this = div()
            .text_color(*colors::TEXT)
            .font_family("Sans")
            .text_size(px(14.0))
            .when(!self.root, |this| {
                this.pl_6().border_l(px(1.0)).border_color(rgb(0x404040))
            });
        match self.element {
            Element::Div(div) => this
                .child("div:")
                .children(div.children.iter().map(|child| TreeviewItem {
                    element: child.clone(),
                    root: false,
                })),
            Element::Text(text) => this.child(format!("\"{text}\"")),
        }
    }
}
