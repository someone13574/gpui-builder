use gpui::*;
use uuid::Uuid;

use crate::{
    appearance::{colors, sizes},
    component::{element::ComponentElement, Component},
};

use super::item::TreeviewItem;

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
    }
}

fn into_list(
    element: ComponentElement,
    indent: u32,
    cx: &mut AppContext,
) -> Vec<(ComponentElement, u32)> {
    let mut flattened = vec![(element.clone(), indent)];
    if let ComponentElement::Div(div) = element {
        let div = cx.read_model(&div, |div, _| div.clone());

        for child in div.children {
            flattened.append(&mut into_list(child, indent + 1, cx));
        }
    }
    flattened
}

fn create_items(
    component: Model<Component>,
    active_element: Model<Option<Uuid>>,
    cx: &mut ViewContext<TreeviewPanel>,
) -> Vec<View<TreeviewItem>> {
    let root_element = cx
        .read_model(&component, |component_root, _cx| component_root.clone())
        .root
        .unwrap();
    let element_list = into_list(root_element, 0, cx);
    element_list
        .into_iter()
        .map(|(element, indent)| TreeviewItem::new(element, active_element.clone(), indent, cx))
        .collect()
}
