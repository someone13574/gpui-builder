use super::element::PreviewElement;
use crate::component::element::div::DivElement;
use gpui::*;

#[derive(IntoElement)]
pub struct PreviewDiv {
    pub div: DivElement,
}

impl RenderOnce for PreviewDiv {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .bg(rgb(0x808080))
            .p_2()
            .m_2()
            .border_1()
            .border_color(white())
            .children(self.div.children.iter().map(|child| PreviewElement {
                element: child.clone(),
            }))
    }
}
