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
        component: &Model<Component>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let root_element = component.read(cx).root.clone();
            let root_item =
                TreeviewItem::new(0, component, root_element.unwrap(), active_element, cx);

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
