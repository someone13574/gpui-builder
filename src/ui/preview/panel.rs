use gpui::*;
use uuid::Uuid;

use super::element::ElementPreview;
use crate::component::Component;

pub struct PreviewPanel {
    root_component: ElementPreview,
}

impl PreviewPanel {
    pub fn new<V: 'static>(
        component: &Component,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let root_component = ElementPreview::new(component, active_id, cx);
            Self { root_component }
        })
    }
}

impl Render for PreviewPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .min_w(px(0.0))
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .child(self.root_component.clone())
    }
}
