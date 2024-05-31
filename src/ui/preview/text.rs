use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use crate::component::element::text::TextElement;

pub struct TextPreview {
    id: Uuid,
    element: Model<TextElement>,
    active_element: Model<Option<Uuid>>,

    indicator_animation_id: Uuid,
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
            cx.observe(&active_element, |this: &mut Self, _, cx| {
                this.indicator_animation_id = Uuid::new_v4();
                cx.notify()
            })
            .detach();

            Self {
                id,
                element,
                active_element,
                indicator_animation_id: Uuid::new_v4(),
            }
        })
    }
}

impl Render for TextPreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_element =
            cx.read_model(&self.active_element, |active_element, _| *active_element);
        let properties = &self.element.read(cx).properties;
        let text = properties
            .iter()
            .find(|property| property.name == "text")
            .unwrap();

        div().child(String::from(text.content.clone())).when(
            active_element == Some(self.id),
            |this| {
                this.child(ActiveIndicator {
                    animation_id: self.indicator_animation_id,
                })
            },
        )
    }
}
