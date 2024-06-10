use gpui::*;
use uuid::Uuid;

use super::item::TreeviewItem;
use crate::appearance::{colors, sizes};
use crate::component::Component;

pub struct TreeviewPanel {
    root_item: View<TreeviewItem>,
    active_id: Model<Option<Uuid>>,
}

impl TreeviewPanel {
    pub fn new<V: 'static>(
        root_component: &Model<Component>,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(root_component, |this: &mut Self, root_component, cx| {
                this.root_item = Self::root_item(&root_component, &this.active_id, cx);
                cx.notify();
            })
            .detach();

            let root_item = Self::root_item(root_component, active_id, cx);
            Self {
                root_item,
                active_id: active_id.clone(),
            }
        })
    }

    fn root_item<V: 'static>(
        root_component: &Model<Component>,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<TreeviewItem> {
        let root_component = root_component.read(cx).clone();
        TreeviewItem::new(0, &root_component, active_id, cx)
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
