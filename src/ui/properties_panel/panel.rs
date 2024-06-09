use gpui::*;
use uuid::Uuid;

use super::property::Property;
use crate::appearance::{colors, sizes};
use crate::component::element::ComponentElement;
use crate::component::Component;

pub struct PropertiesPanel {
    component: Model<Component>,
    properties: Vec<Property>,
    scroll_handle: ScrollHandle,
}

impl PropertiesPanel {
    pub fn new<V: 'static>(
        component: Model<Component>,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_active_element = *active_element.read(cx);
            let properties = if let Some(active_element) = get_active_element(
                component.read(cx).root.as_ref().unwrap(),
                cached_active_element,
                cx,
            ) {
                Self::make_properties(&active_element, cx)
            } else {
                Vec::new()
            };

            Self::observe_active_element(&active_element, cx);

            Self {
                component,
                properties,
                scroll_handle: ScrollHandle::new(),
            }
        })
    }

    fn make_properties(element: &ComponentElement, cx: &mut ViewContext<Self>) -> Vec<Property> {
        let properties = match element {
            ComponentElement::Div(element) => element.properties.clone(),
            ComponentElement::Text(element) => element.properties.clone(),
        };

        properties
            .into_iter()
            .map(|(key, property)| Property::new(property, key, cx))
            .collect()
    }

    fn observe_active_element(active_element: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_element, |this, active_element_id, cx| {
            let active_element_id = *active_element_id.read(cx);
            this.properties = if let Some(active_element) = get_active_element(
                this.component.read(cx).root.as_ref().unwrap(),
                active_element_id,
                cx,
            ) {
                Self::make_properties(&active_element, cx)
            } else {
                Vec::new()
            };
            cx.notify();
        })
        .detach();
    }
}

impl Render for PropertiesPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .bg(*colors::SIDEBAR_BG)
            .min_w(*sizes::SIDEBAR_WIDTH)
            .border_l_1()
            .border_color(*colors::BORDER)
            .children(self.properties.clone())
            .overflow_x_hidden()
            .id("properties_panel")
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
    }
}

fn get_active_element(
    search_element: &ComponentElement,
    active_element: Option<Uuid>,
    cx: &AppContext,
) -> Option<ComponentElement> {
    if Some(search_element.id()) == active_element {
        return Some(search_element.clone());
    }

    match search_element {
        ComponentElement::Div(element) => {
            let children = element.children.read(cx);
            children
                .iter()
                .find_map(|child| get_active_element(child, active_element, cx))
        }
        ComponentElement::Text(_) => None,
    }
}
