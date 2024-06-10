use gpui::*;
use indexmap::IndexMap;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use super::component::ComponentPreview;
use crate::component::div::DivComponent;
use crate::component::property::{enum_prop, read_properties, ComponentProperty};
use crate::component::Component;

pub struct DivPreview {
    component: DivComponent,
    active_id: Model<Option<Uuid>>,

    cached_properties: IndexMap<String, ComponentProperty>,
    child_previews: Vec<ComponentPreview>,
    indicator_animation_id: Option<Uuid>,
}

impl DivPreview {
    pub fn new<V: 'static>(
        component: &DivComponent,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_properties = read_properties(&component.properties, cx);
            let child_previews = create_child_previews(&component.children, active_id, cx);
            let indicator_animation_id = if *active_id.read(cx) == Some(component.id) {
                Some(Uuid::new_v4())
            } else {
                None
            };

            Self::observe_properties(component, cx);
            Self::observe_children(component, cx);
            Self::observe_active_id(active_id, cx);

            Self {
                component: component.clone(),
                active_id: active_id.clone(),

                cached_properties,
                child_previews,
                indicator_animation_id,
            }
        })
    }

    fn observe_properties(component: &DivComponent, cx: &mut ViewContext<Self>) {
        for (key, value) in &component.properties {
            let key = key.clone();
            cx.observe(value, move |this, property, cx| {
                this.cached_properties
                    .insert(key.clone(), property.read(cx).clone());
                cx.notify();
            })
            .detach();
        }
    }

    fn observe_children(component: &DivComponent, cx: &mut ViewContext<Self>) {
        cx.observe(&component.children, |this, children, cx| {
            this.child_previews = create_child_previews(&children, &this.active_id, cx);
            cx.notify();
        })
        .detach();
    }

    fn observe_active_id(active_id: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_id, |this, active_id, cx| {
            let active_id = *active_id.read(cx);
            if active_id == Some(this.component.id) {
                this.indicator_animation_id = Some(Uuid::new_v4());
            } else {
                this.indicator_animation_id = None;
            }
            cx.notify();
        })
        .detach();
    }

    fn get_property(&self, key: &str) -> ComponentProperty {
        self.cached_properties.get(key).unwrap().clone()
    }

    fn get_enum_property<T: 'static>(&self, key: &str) -> T {
        enum_prop::EnumProperty::from(self.get_property(key)).value()
    }
}

impl Render for DivPreview {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let display: Display = self.get_enum_property("display");
        let overflow_x: Overflow = self.get_enum_property("overflow_x");
        let overflow_y: Overflow = self.get_enum_property("overflow_y");

        div()
            .id("preview-div")
            .when(display == Display::Block, |this| this.block())
            .when(display == Display::Flex, |this| this.flex())
            .when(!bool::from(self.get_property("visible")), |this| {
                this.invisible()
            })
            .when(overflow_x == Overflow::Hidden, |this| {
                this.overflow_x_hidden()
            })
            .when(overflow_x == Overflow::Scroll, |this| {
                this.overflow_x_scroll()
            })
            .when(overflow_y == Overflow::Hidden, |this| {
                this.overflow_y_hidden()
            })
            .when(overflow_y == Overflow::Scroll, |this| {
                this.overflow_y_scroll()
            })
            .ml(px(self.get_property("margin_left").into()))
            .mr(px(self.get_property("margin_right").into()))
            .mt(px(self.get_property("margin_top").into()))
            .mb(px(self.get_property("margin_bottom").into()))
            .pl(px(self.get_property("padding_left").into()))
            .pr(px(self.get_property("padding_right").into()))
            .pt(px(self.get_property("padding_top").into()))
            .pb(px(self.get_property("padding_bottom").into()))
            .border_l(px(self.get_property("border_left").into()))
            .border_r(px(self.get_property("border_right").into()))
            .border_t(px(self.get_property("border_top").into()))
            .border_b(px(self.get_property("border_bottom").into()))
            .gap_x(px(self.get_property("gap_x").into()))
            .gap_y(px(self.get_property("gap_y").into()))
            .bg(Rgba::from(self.get_property("background")))
            .border_color(Rgba::from(self.get_property("border_color")))
            .rounded_tl(px(self.get_property("radius_top_left").into()))
            .rounded_tr(px(self.get_property("radius_top_right").into()))
            .rounded_bl(px(self.get_property("radius_bottom_left").into()))
            .rounded_br(px(self.get_property("radius_bottom_right").into()))
            .cursor(self.get_enum_property("cursor_style"))
            .children(self.child_previews.clone())
            .when_some(self.indicator_animation_id, |this, animation_id| {
                this.child(ActiveIndicator { animation_id })
            })
    }
}

fn create_child_previews(
    children: &Model<Vec<Component>>,
    active_id: &Model<Option<Uuid>>,
    cx: &mut ViewContext<DivPreview>,
) -> Vec<ComponentPreview> {
    let children = children.read(cx).clone();
    children
        .iter()
        .map(|child| ComponentPreview::new(child, active_id, cx))
        .collect()
}
