use gpui::*;
use uuid::Uuid;

use crate::{appearance::colors, component::Component};

use super::element::PreviewElement;

pub struct PreviewPanel {
    component: Model<Component>,
    active_element: Model<Option<Uuid>>,
}

impl PreviewPanel {
    pub fn new<V: 'static>(
        component: Model<Component>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&active_element, |_, _, cx| {
                cx.notify();
            })
            .detach();

            Self {
                component,
                active_element,
            }
        })
    }
}

impl Render for PreviewPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.component, |component, _| component.clone())
            .root;

        let active_element = cx.read_model(&self.active_element, |id, _| *id);

        div()
            .flex()
            .size_full()
            .min_w_0()
            .overflow_hidden()
            .items_center()
            .justify_center()
            .bg(*colors::BG)
            .child(PreviewElement {
                element: root_element,
                active_element,
            })
    }
}
