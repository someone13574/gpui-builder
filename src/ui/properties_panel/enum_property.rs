use gpui::*;
use prelude::FluentBuilder;

use crate::appearance::colors;
use crate::component::property::{enum_prop, ComponentProperty};

pub struct EnumProperty {
    property_name: String,

    expanded: bool,
    active_item: String,
    items: Vec<View<EnumItem>>,
}

impl EnumProperty {
    pub fn new<V: 'static>(
        property: Model<ComponentProperty>,
        property_name: String,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let property_value: enum_prop::EnumProperty = property.read(cx).clone().into();

            let items = property_value
                .valid
                .into_iter()
                .map(|item| EnumItem::new(item, property.clone(), cx))
                .collect();

            cx.observe(&property, |this: &mut Self, property, cx| {
                this.active_item = enum_prop::EnumProperty::from(property.read(cx).clone()).value;
                this.expanded = false;
                cx.notify();
            })
            .detach();

            Self {
                property_name,
                expanded: false,
                active_item: property_value.value,
                items,
            }
        })
    }
}

impl Render for EnumProperty {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(format!("{}: ", self.property_name))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .border_1()
                    .border_color(*colors::BORDER)
                    .rounded(px(8.0))
                    .gap_1()
                    .px_1()
                    .w_full()
                    .id("enum-dropdown")
                    .on_click(cx.listener(|this, _, cx| {
                        this.expanded = true;
                        cx.notify();
                    }))
                    .when(!self.expanded, |this| this.child(self.active_item.clone()))
                    .when(self.expanded, |this| this.children(self.items.clone())),
            )
    }
}

struct EnumItem {
    text: String,
    property: Model<ComponentProperty>,
}

impl EnumItem {
    pub fn new(
        text: String,
        property: Model<ComponentProperty>,
        cx: &mut ViewContext<EnumProperty>,
    ) -> View<Self> {
        cx.new_view(|_| Self { text, property })
    }
}

impl Render for EnumItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .child(self.text.clone())
            .id("item")
            .on_click(cx.listener(|this, _, cx| {
                this.property.update(cx, |property, cx| {
                    let mut enum_property: enum_prop::EnumProperty = property.clone().into();
                    enum_property.value.clone_from(&this.text);
                    *property = enum_property.into();
                    cx.notify();
                })
            }))
    }
}
