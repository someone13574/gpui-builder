use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::element::ElementPreview;
use crate::component::Component;

pub struct PreviewPanel {
    root_element: Option<ElementPreview>,
}

impl PreviewPanel {
    pub fn new<V: 'static>(
        component: Model<Component>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let root_element = cx
                .read_model(&component, |component, _| component.clone())
                .root;
            let root_element = root_element
                .map(|root_element| ElementPreview::new(root_element, active_element, cx));
            Self { root_element }
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
            .when_some(self.root_element.clone(), |this, root_element| {
                this.child(root_element)
            })
    }
}
