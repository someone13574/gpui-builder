use gpui::*;
use uuid::Uuid;

use super::div::DivPreview;
use super::text::TextPreview;
use crate::component::element::ComponentElement;

#[derive(IntoElement, Clone)]
pub enum ElementPreview {
    Div(View<DivPreview>),
    Text(View<TextPreview>),
}

impl ElementPreview {
    pub fn new<V: 'static>(
        element: ComponentElement,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match element {
            ComponentElement::Div(element) => {
                Self::Div(DivPreview::new(element, active_element, cx))
            }
            ComponentElement::Text(element) => {
                Self::Text(TextPreview::new(element, active_element, cx))
            }
        }
    }
}

impl RenderOnce for ElementPreview {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            ElementPreview::Div(element) => div().child(element),
            ElementPreview::Text(element) => div().child(element),
        }
    }
}
