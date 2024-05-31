use gpui::*;

use crate::appearance::{colors, sizes};

pub struct PropertiesPanel {}

impl PropertiesPanel {
    pub fn new<V: 'static>(cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|_cx| Self {})
    }
}

impl Render for PropertiesPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .bg(*colors::SIDEBAR_BG)
            .min_w(*sizes::SIDEBAR_WIDTH)
            .border_l_1()
            .border_color(*colors::BORDER)
    }
}
