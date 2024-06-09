use gpui::*;
use indexmap::IndexMap;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use super::element::ElementPreview;
use crate::component::element::div::DivElement;
use crate::component::element::ComponentElement;
use crate::component::element_property::{enum_property, read_properties, ElementProperty};

pub struct DivPreview {
    element: DivElement,
    active_element: Model<Option<Uuid>>,

    cached_properties: IndexMap<String, ElementProperty>,
    child_previews: Vec<ElementPreview>,
    indicator_animation_id: Option<Uuid>,
}

impl DivPreview {
    pub fn new<V: 'static>(
        element: DivElement,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_properties = read_properties(&element.properties, cx);
            let child_previews = create_child_previews(&element.children, &active_element, cx);
            let indicator_animation_id = if *active_element.read(cx) == Some(element.id) {
                Some(Uuid::new_v4())
            } else {
                None
            };

            Self::observe_properties(&element, cx);
            Self::observe_children(&element, cx);
            Self::observe_active_element(&active_element, cx);

            Self {
                element,
                active_element,

                cached_properties,
                child_previews,
                indicator_animation_id,
            }
        })
    }

    fn observe_properties(element: &DivElement, cx: &mut ViewContext<Self>) {
        for (_, value) in &element.properties {
            cx.observe(value, |this, property, cx| {
                let (key, value) = property.read(cx).clone();
                this.cached_properties.insert(key, value);
                cx.notify();
            })
            .detach();
        }
    }

    fn observe_children(element: &DivElement, cx: &mut ViewContext<Self>) {
        cx.observe(&element.children, |this, children, cx| {
            this.child_previews = create_child_previews(&children, &this.active_element, cx);
            cx.notify();
        })
        .detach();
    }

    fn observe_active_element(active_element: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_element, |this, active_element, cx| {
            let active_element = *active_element.read(cx);
            if active_element == Some(this.element.id) {
                this.indicator_animation_id = Some(Uuid::new_v4());
            } else {
                this.indicator_animation_id = None;
            }
            cx.notify();
        })
        .detach();
    }

    fn get_property(&self, key: &str) -> ElementProperty {
        self.cached_properties.get(key).unwrap().clone()
    }

    fn get_enum_property<T: 'static>(&self, key: &str) -> T {
        enum_property::EnumProperty::from(self.get_property(key)).value()
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
    children: &Model<Vec<ComponentElement>>,
    active_element: &Model<Option<Uuid>>,
    cx: &mut ViewContext<DivPreview>,
) -> Vec<ElementPreview> {
    let children = children.read(cx).clone();
    children
        .into_iter()
        .map(|child| ElementPreview::new(child, active_element.clone(), cx))
        .collect()
}
