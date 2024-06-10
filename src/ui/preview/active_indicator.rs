use std::time::Duration;

use gpui::*;
use uuid::Uuid;

#[derive(IntoElement)]
pub struct ActiveIndicator {
    pub animation_id: Uuid,
}

impl RenderOnce for ActiveIndicator {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().absolute().inset_0().with_animation(
            self.animation_id,
            Animation::new(Duration::from_millis(500)).with_easing(bounce(ease_in_out)),
            |this, delta| {
                let mut color = red();
                color.a *= delta;
                this.bg(color)
            },
        )
    }
}
