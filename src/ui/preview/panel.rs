use gpui::*;
use uuid::Uuid;

use super::component::ComponentPreview;
use crate::component::Component;

pub struct PreviewPanel {
    root_component: ComponentPreview,
    active_id: Model<Option<Uuid>>,
}

impl PreviewPanel {
    pub fn new<V: 'static>(
        root_component: &Model<Component>,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(root_component, |this: &mut Self, root_component, cx| {
                this.root_component = Self::root_component(&root_component, &this.active_id, cx);
                cx.notify();
            })
            .detach();

            let root_component = Self::root_component(root_component, active_id, cx);
            Self {
                root_component,
                active_id: active_id.clone(),
            }
        })
    }

    fn root_component<V: 'static>(
        root_component: &Model<Component>,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> ComponentPreview {
        let root_component = root_component.read(cx).clone();
        ComponentPreview::new(&root_component, active_id, cx)
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
