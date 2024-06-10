use gpui::*;
use uuid::Uuid;

use super::div::DivPreview;
use super::text::TextPreview;
use crate::component::Component;

#[derive(IntoElement, Clone)]
pub enum ComponentPreview {
    Div(View<DivPreview>),
    Text(View<TextPreview>),
}

impl ComponentPreview {
    pub fn new<V: 'static>(
        component: &Component,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> Self {
        match component {
            Component::Div(component) => Self::Div(DivPreview::new(component, active_id, cx)),
            Component::Text(component) => Self::Text(TextPreview::new(component, active_id, cx)),
        }
    }
}

impl RenderOnce for ComponentPreview {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self {
            ComponentPreview::Div(component) => div().child(component),
            ComponentPreview::Text(component) => div().child(component),
        }
    }
}
