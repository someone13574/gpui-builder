use crate::component::element::text::TextElement;
use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;

pub struct TextPreview {
    id: Uuid,
    element: Model<TextElement>,
    active_element: Model<Option<Uuid>>,
}

impl TextPreview {
    pub fn new<V: 'static>(
        element: Model<TextElement>,
        id: Uuid,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&element, |_, _, cx| cx.notify()).detach();
            cx.observe(&active_element, |_, _, cx| cx.notify()).detach();

            Self {
                id,
                element,
                active_element,
            }
        })
    }
}

impl Render for TextPreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text_element = cx.read_model(&self.element, |element, _| element.clone());
        let active_element =
            cx.read_model(&self.active_element, |active_element, _| *active_element);

        div()
            .child(text_element.text)
            .when(active_element == Some(self.id), |this| {
                this.child(ActiveIndicator {})
            })
    }
}
