use std::time::Duration;

use gpui::*;

use crate::appearance::colors;

#[derive(IntoElement)]
pub struct ActiveIndicator;

impl RenderOnce for ActiveIndicator {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().absolute().inset_0().with_animation(
            "active-indicator",
            Animation::new(Duration::from_millis(500)).with_easing(bounce(ease_in_out)),
            |this, delta| {
                let mut color = *colors::ACTIVE_ELEMENT_INDICATOR_MAX;
                color.a *= delta;
                this.bg(color)
            },
        )
    }
}
