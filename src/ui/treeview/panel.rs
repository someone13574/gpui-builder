use gpui::*;
use uuid::Uuid;

use super::item::TreeviewItem;
use crate::appearance::{colors, sizes};
use crate::component::Component;

pub struct TreeviewPanel {
    active_element: Model<Option<Uuid>>,
    item_views: Vec<View<TreeviewItem>>,
}

impl TreeviewPanel {
    pub fn new<V: 'static>(
        component: Model<Component>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&component, |this: &mut TreeviewPanel, component, cx| {
                this.item_views = create_items(component, this.active_element.clone(), cx);
                cx.notify();
            })
            .detach();

            let item_views = create_items(component.clone(), active_element.clone(), cx);
            Self {
                active_element,
                item_views,
            }
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
            .children(self.item_views.clone())
            .overflow_hidden()
    }
}

fn create_items(
    component: Model<Component>,
    active_element: Model<Option<Uuid>>,
    cx: &mut ViewContext<TreeviewPanel>,
) -> Vec<View<TreeviewItem>> {
    let component = cx.read_model(&component, |component_root, _cx| component_root.clone());
    let element_list = component.element_list(cx);
    element_list
        .into_iter()
        .map(|(element, indent)| TreeviewItem::new(element, active_element.clone(), indent, cx))
        .collect()
}
