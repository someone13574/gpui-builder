use gpui::*;
use prelude::FluentBuilder;

use crate::{colors, hierarchy::HierarchyDiv};

pub struct ComponentPreviewView {
    pub hierarchy_model: Model<HierarchyDiv>,
}

impl Render for ComponentPreviewView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.observe(&self.hierarchy_model, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let root_element = cx.read_model(&self.hierarchy_model, |root, _| root.clone());

        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(*colors::BACKGROUND)
            .child(ComponentPreviewDivElement::from(root_element))
    }
}

#[derive(IntoElement)]
struct ComponentPreviewDivElement {
    div_element: HierarchyDiv,
}

impl From<HierarchyDiv> for ComponentPreviewDivElement {
    fn from(value: HierarchyDiv) -> Self {
        Self { div_element: value }
    }
}

impl RenderOnce for ComponentPreviewDivElement {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .m_2()
            .border_1()
            .rounded(px(8.0))
            .border_color(rgb(0xffffff))
            .when_some(self.div_element.text, |this, text| this.child(text))
            .when(!self.div_element.children.is_empty(), |mut this| {
                for child in &self.div_element.children {
                    this = this.child(ComponentPreviewDivElement::from(child.clone()));
                }
                this
            })
            .bg(rgb(0x808080))
    }
}
