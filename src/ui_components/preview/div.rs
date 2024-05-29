use gpui::prelude::*;
use gpui::*;

use crate::hierarchy::{DivElement, Element};

#[derive(IntoElement)]
pub struct DivPreview {
    element: DivElement,
}

impl From<DivElement> for DivPreview {
    fn from(value: DivElement) -> Self {
        Self { element: value }
    }
}

impl RenderOnce for DivPreview {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let mut this = div()
            .p_2()
            .m_2()
            .border_1()
            .rounded(px(16.0))
            .border_color(rgb(0xffffff))
            .bg(rgb(0x808080))
            .font_family("Sans");

        for child in self.element.children {
            match child {
                Element::Div(div) => this = this.child(DivPreview::from(div)),
                Element::Text(text) => this = this.child(text),
            }
        }
        this
    }
}
