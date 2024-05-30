use gpui::*;
use uuid::Uuid;

use crate::appearance::{colors, sizes};

pub struct PropertiesPanel {
    _active_element: Model<Option<Uuid>>,
}

impl PropertiesPanel {
    pub fn new<V: 'static>(
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&active_element, |_, _, cx| cx.notify()).detach();
            Self {
                _active_element: active_element,
            }
        })
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
