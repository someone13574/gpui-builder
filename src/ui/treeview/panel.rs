use gpui::*;
use uuid::Uuid;

use super::item::TreeviewItem;
use crate::appearance::{colors, sizes};
use crate::component::element::ComponentElement;
use crate::component::Component;

pub struct TreeviewPanel {
    root_component: ComponentElement,
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
            let root_component = component.read(cx).root.clone().unwrap();
            let item_views = create_items(root_component.clone(), active_element.clone(), cx);

            Self {
                root_component,
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
    root: ComponentElement,
    active_element: Model<Option<Uuid>>,
    cx: &mut ViewContext<TreeviewPanel>,
) -> Vec<View<TreeviewItem>> {
    let element_list = root.element_list(0, cx);

    for (element, _) in &element_list {
        match element {
            ComponentElement::Div(element) => {
                cx.observe(element, |this, _, cx| {
                    this.item_views =
                        create_items(this.root_component.clone(), this.active_element.clone(), cx);
                    cx.notify();
                })
                .detach()
            }
            ComponentElement::Text(element) => {
                cx.observe(element, |this, _, cx| {
                    this.item_views =
                        create_items(this.root_component.clone(), this.active_element.clone(), cx);
                    cx.notify();
                })
                .detach()
            }
        }
    }

    element_list
        .into_iter()
        .map(|(element, indent)| TreeviewItem::new(element, active_element.clone(), indent, cx))
        .collect()
}
