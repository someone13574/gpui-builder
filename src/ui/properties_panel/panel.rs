use gpui::*;
use uuid::Uuid;

use super::property::Property;
use crate::appearance::{colors, sizes};
use crate::component::Component;

pub struct PropertiesPanel {
    root_component: Model<Component>,
    properties: Vec<Property>,
    scroll_handle: ScrollHandle,
}

impl PropertiesPanel {
    pub fn new<V: 'static>(
        root_component: &Model<Component>,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_active_id = *active_id.read(cx);
            let cached_root_component = root_component.read(cx).clone();
            let properties = if let Some(active) =
                get_active_component(&cached_root_component, cached_active_id, cx)
            {
                Self::make_properties(&active, cx)
            } else {
                Vec::new()
            };

            Self::observe_active_id(active_id, cx);
            Self::observe_root_component(root_component, cx);

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
            .map(|(key, (_, property))| Property::new(property, key, cx))
            .collect()
    }

    fn observe_active_id(active_id: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_id, |this, active_id, cx| {
            let active_id = *active_id.read(cx);
            let cached_root_component = this.root_component.read(cx).clone();
            this.properties = if let Some(active_component) =
                get_active_component(&cached_root_component, active_id, cx)
            {
                Self::make_properties(&active_component, cx)
            } else {
                Vec::new()
            };
            cx.notify();
        })
        .detach();
    }

    fn observe_root_component(root_component: &Model<Component>, cx: &mut ViewContext<Self>) {
        cx.observe(root_component, |this: &mut Self, _, cx| {
            let cached_root_component = this.root_component.read(cx).clone();
            this.properties = Self::make_properties(&cached_root_component, cx);
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

fn get_active_component(
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
                .find_map(|child| get_active_component(child, active_id, cx))
        }
        Component::Text(_) => None,
    }
}
