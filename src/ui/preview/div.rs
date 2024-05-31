use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use super::element::ElementPreview;
use crate::component::element::div::DivElement;

pub struct DivPreview {
    id: Uuid,
    children: Vec<ElementPreview>,
    active_element: Model<Option<Uuid>>,

    indicator_animation_id: Uuid,
}

impl DivPreview {
    pub fn new<V: 'static>(
        element: Model<DivElement>,
        id: Uuid,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&element, |this: &mut DivPreview, element, cx| {
                this.children = Self::make_children(element, this.active_element.clone(), cx);
                cx.notify()
            })
            .detach();
            cx.observe(&active_element, |this, _, cx| {
                this.indicator_animation_id = Uuid::new_v4();
                cx.notify();
            })
            .detach();

            let children = Self::make_children(element.clone(), active_element.clone(), cx);
            Self {
                id,
                children,
                active_element,
                indicator_animation_id: Uuid::new_v4(),
            }
        })
    }

    fn make_children(
        element: Model<DivElement>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<Self>,
    ) -> Vec<ElementPreview> {
        let element = cx.read_model(&element, |element, _| element.clone());
        element
            .children
            .into_iter()
            .map(|element| ElementPreview::new(element, active_element.clone(), cx))
            .collect()
    }
}

impl Render for DivPreview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_element =
            cx.read_model(&self.active_element, |active_element, _| *active_element);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .rounded(px(16.0))
            .bg(rgb(0x808080))
            .border_color(white())
            .border_1()
            .children(self.children.clone())
            .when(active_element == Some(self.id), |this| {
                this.child(ActiveIndicator {
                    animation_id: self.indicator_animation_id,
                })
            })
    }
}
