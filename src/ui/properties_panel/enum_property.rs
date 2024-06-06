use gpui::*;
use prelude::FluentBuilder;

use crate::{
    appearance::colors,
    component::element_property::{enum_property, ElementProperty},
};

pub struct EnumProperty {
    property_name: String,

    expanded: bool,
    active_item: String,
    items: Vec<View<EnumItem>>,
}

impl EnumProperty {
    pub fn new<V: 'static>(
        property: Model<(String, ElementProperty)>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let (property_name, property_value) = property.read(cx).clone();
            let property_value: enum_property::EnumProperty = property_value.into();

            let items = property_value
                .valid
                .into_iter()
                .map(|item| EnumItem::new(item, property.clone(), cx))
                .collect();

            cx.observe(&property, |this: &mut Self, property, cx| {
                this.active_item =
                    enum_property::EnumProperty::from(property.read(cx).1.clone()).value;
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
    property: Model<(String, ElementProperty)>,
}

impl EnumItem {
    pub fn new(
        text: String,
        property: Model<(String, ElementProperty)>,
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
                this.property.update(cx, |(_, property), cx| {
                    let mut enum_property: enum_property::EnumProperty = property.clone().into();
                    enum_property.value = this.text.clone();
                    *property = enum_property.into();
                    cx.notify();
                })
            }))
    }
}
