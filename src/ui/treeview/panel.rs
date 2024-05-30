use gpui::*;

use crate::{
    appearance::{colors, sizes},
    component::{element::ComponentElement, Component},
};

use super::item::TreeviewItem;

pub struct TreeviewPanel {
    pub component: Model<Component>,
    pub item_views: Vec<View<TreeviewItem>>,
}

impl TreeviewPanel {
    pub fn new<V: 'static>(component: Model<Component>, cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&component, |this: &mut TreeviewPanel, component, cx| {
                this.item_views = list_item_views(component, cx);
                cx.notify();
            })
            .detach();

            let item_views = list_item_views(component.clone(), cx);
            Self {
                component,
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

fn into_list(element: ComponentElement, indent: u32) -> Vec<(ComponentElement, u32)> {
    let mut flattened = vec![(element.clone(), indent)];
    if let ComponentElement::Div(div) = element {
        for child in div.children {
            flattened.append(&mut into_list(child, indent + 1));
        }
    }
    flattened
}

fn list_item_views(
    component: Model<Component>,
    cx: &mut ViewContext<TreeviewPanel>,
) -> Vec<View<TreeviewItem>> {
    let root_element = cx
        .read_model(&component, |component_root, _cx| component_root.clone())
        .root;
    let element_list = into_list(root_element, 0);
    element_list
        .into_iter()
        .map(|(element, indent)| TreeviewItem::new(element, indent, cx))
        .collect()
}
