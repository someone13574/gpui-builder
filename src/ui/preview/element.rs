use super::div::PreviewDiv;
use crate::{appearance::colors, component::element::ComponentElement};
use gpui::*;
use prelude::FluentBuilder;
use std::time::Duration;
use uuid::Uuid;

#[derive(IntoElement)]
pub struct PreviewElement {
    pub element: ComponentElement,
    pub active_element: Option<Uuid>,
}

impl RenderOnce for PreviewElement {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        match self.element.clone() {
            ComponentElement::Div(element) => div().child(PreviewDiv {
                div: element,
                active_element: self.active_element,
            }),
            ComponentElement::Text(element) => div().child(element.text),
        }
        .when(self.active_element == self.element.id(), |this| {
            this.child(div().absolute().size_full().inset_0().with_animation(
                self.element.id().unwrap(),
                Animation::new(Duration::from_millis(500)).with_easing(bounce(ease_in_out)),
                |this, delta| {
                    let mut color = *colors::ACTIVE_ELEMENT_INDICATOR_MAX;
                    color.a *= delta;
                    this.bg(color)
                },
            ))
        })
    }
}
