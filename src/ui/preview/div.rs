use super::element::PreviewElement;
use crate::component::element::div::DivElement;
use gpui::*;
use uuid::Uuid;

#[derive(IntoElement)]
pub struct PreviewDiv {
    pub div: DivElement,
    pub active_element: Option<Uuid>,
}

impl RenderOnce for PreviewDiv {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .bg(rgb(0x808080))
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .border_1()
            .border_color(white())
            .children(self.div.children.iter().map(|child| PreviewElement {
                element: child.clone(),
                active_element: self.active_element,
            }))
    }
}
