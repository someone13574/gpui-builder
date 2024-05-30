use crate::component::element::ComponentElement;
use gpui::*;

use super::div::PreviewDiv;

#[derive(IntoElement)]
pub struct PreviewElement {
    pub element: ComponentElement,
}

impl RenderOnce for PreviewElement {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self.element {
            ComponentElement::Div(element) => div().child(PreviewDiv { div: element }),
            ComponentElement::Text(element) => div().child(element.0),
        }
    }
}
