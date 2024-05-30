use gpui::*;

use crate::{appearance::colors, component::Component};

use super::element::PreviewElement;

pub struct PreviewPanel {
    component: Model<Component>,
}

impl PreviewPanel {
    pub fn new<V: 'static>(component: Model<Component>, cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|_| Self { component })
    }
}

impl Render for PreviewPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.component, |component, _| component.clone())
            .root;

        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .bg(*colors::BG)
            .child(PreviewElement {
                element: root_element,
            })
    }
}
