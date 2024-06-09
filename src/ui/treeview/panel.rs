use gpui::*;
use uuid::Uuid;

use super::item::TreeviewItem;
use crate::appearance::{colors, sizes};
use crate::component::Component;

pub struct TreeviewPanel {
    root_item: View<TreeviewItem>,
}

impl TreeviewPanel {
    pub fn new<V: 'static>(
        root_component: &Component,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let root_item = TreeviewItem::new(0, root_component, active_id, cx);

            Self { root_item }
        })
    }
}

impl Render for TreeviewPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .bg(*colors::SIDEBAR_BG)
            .min_w(*sizes::SIDEBAR_WIDTH)
            .border_r_1()
            .border_color(*colors::BORDER)
            .child(self.root_item.clone())
            .overflow_hidden()
    }
}
