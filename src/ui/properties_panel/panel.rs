use gpui::*;
use uuid::Uuid;

use super::property::Property;
use crate::appearance::{colors, sizes};
use crate::component::Component;

pub struct PropertiesPanel {
    root_component: Component,
    properties: Vec<Property>,
    scroll_handle: ScrollHandle,
}

impl PropertiesPanel {
    pub fn new<V: 'static>(
        root_component: &Component,
        active_element: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_active_element = *active_element.read(cx);
            let properties = if let Some(active_element) =
                get_active_element(root_component, cached_active_element, cx)
            {
                Self::make_properties(&active_element, cx)
            } else {
                Vec::new()
            };

            Self::observe_active_element(active_element, cx);

            Self {
                root_component: root_component.clone(),
                properties,
                scroll_handle: ScrollHandle::new(),
            }
        })
    }

    fn make_properties(component: &Component, cx: &mut ViewContext<Self>) -> Vec<Property> {
        let properties = match component {
            Component::Div(component) => component.properties.clone(),
            Component::Text(component) => component.properties.clone(),
        };

        properties
            .into_iter()
            .map(|(key, property)| Property::new(property, key, cx))
            .collect()
    }

    fn observe_active_element(active_element: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_element, |this, active_element_id, cx| {
            let active_element_id = *active_element_id.read(cx);
            this.properties = if let Some(active_element) =
                get_active_element(&this.root_component, active_element_id, cx)
            {
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
    search_component: &Component,
    active_id: Option<Uuid>,
    cx: &AppContext,
) -> Option<Component> {
    if Some(search_component.id()) == active_id {
        return Some(search_component.clone());
    }

    match search_component {
        Component::Div(component) => {
            let children = component.children.read(cx);
            children
                .iter()
                .find_map(|child| get_active_element(child, active_id, cx))
        }
        Component::Text(_) => None,
    }
}
