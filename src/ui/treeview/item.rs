use gpui::prelude::*;
use gpui::*;
use uuid::Uuid;

use super::context_actions::item_context_actions;
use crate::appearance::{colors, sizes};
use crate::component::Component;
use crate::ui::context_menu::ContextMenuGlobal;

pub struct TreeviewItem {
    indent: u32,
    pub component: Component,
    pub active_id: Model<Option<Uuid>>,

    cached_text: String,
    child_views: Vec<View<TreeviewItem>>,

    hovered: bool,
    active: bool,
}

impl TreeviewItem {
    pub fn new<V: 'static>(
        indent: u32,
        component: &Component,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_text = get_text(component, cx);
            let active = *active_id.read(cx) == Some(component.id());

            Self::observe_component(component, cx);
            Self::observe_active_id(active_id, cx);

            let mut this = Self {
                indent,
                component: component.clone(),
                active_id: active_id.clone(),

                cached_text,
                child_views: Vec::new(),

                hovered: false,
                active,
            };
            this.make_child_views(cx);
            this
        })
    }

    fn make_child_views(&mut self, cx: &mut ViewContext<Self>) {
        if let Component::Div(div_component) = &self.component {
            let children = div_component.children.read(cx).clone();
            self.child_views = children
                .iter()
                .map(|child| TreeviewItem::new(self.indent + 1, child, &self.active_id, cx))
                .collect();
        }
    }

    fn observe_component(component: &Component, cx: &mut ViewContext<Self>) {
        match component {
            Component::Div(component) => cx
                .observe(&component.children, |this, _, cx| {
                    this.make_child_views(cx);
                    cx.notify();
                })
                .detach(),
            Component::Text(component) => {
                cx.observe(component.properties.get("text").unwrap(), |this, _, cx| {
                    this.cached_text = get_text(&this.component, cx);
                    cx.notify();
                })
                .detach();
            }
        }
    }

    fn observe_active_id(active_id: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_id, |this, active_id, cx| {
            this.active = Some(this.component.id()) == *active_id.read(cx);
            cx.notify();
        })
        .detach();
    }
}

impl Render for TreeviewItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .px_2()
                    .when(self.hovered, |this| this.bg(*colors::LIST_ITEM_HOVER))
                    .when(self.active, |this| this.bg(*colors::LIST_ITEM_ACTIVE))
                    .children((0..self.indent).map(|_| {
                        div()
                            .min_w(*sizes::TREEVIEW_INDENT)
                            .border_l_1()
                            .border_color(*colors::BORDER)
                    }))
                    .child(self.cached_text.clone())
                    .id("treeview-item")
                    .on_mouse_up(
                        MouseButton::Right,
                        cx.listener(|this, event: &MouseUpEvent, cx| {
                            ContextMenuGlobal::activate(
                                event.position,
                                item_context_actions(this, cx),
                                cx,
                            )
                        }),
                    )
                    .on_click(cx.listener(|this, event: &ClickEvent, cx| {
                        if event.down.button == MouseButton::Left {
                            cx.update_model(&this.active_id, |active_id, cx| {
                                *active_id = Some(this.component.id());
                                cx.notify();
                            })
                        }
                    }))
                    .on_hover(cx.listener(|this, hover, cx| {
                        this.hovered = *hover;
                        cx.notify();
                    })),
            )
            .children(self.child_views.clone())
    }
}

fn get_text(component: &Component, cx: &AppContext) -> String {
    match component {
        Component::Div(_) => "div:".to_string(),
        Component::Text(component) => {
            let text_property = component.properties.get("text").unwrap().read(cx).clone();
            let text_property: String = text_property.into();
            format!("\"{text_property}\"")
        }
    }
}
