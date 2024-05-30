use crate::{
    appearance::{colors, sizes},
    component::element::ComponentElement,
};
use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

pub struct TreeviewItem {
    element: ComponentElement,
    active_element: Model<Option<Uuid>>,
    indent: u32,

    hover: bool,
    active: bool,
}

impl TreeviewItem {
    pub fn new<V: 'static>(
        element: ComponentElement,
        active_element: Model<Option<Uuid>>,
        indent: u32,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&active_element, |this: &mut Self, active_element, cx| {
                let active_element =
                    cx.read_model(&active_element, |active_element, _| *active_element);
                this.active = this.element.id() == active_element;
                cx.notify();
            })
            .detach();

            Self {
                element,
                active_element,
                indent,
                hover: false,
                active: false,
            }
        })
    }
}

impl Render for TreeviewItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .px_2()
            .children((0..self.indent).map(|_| {
                div()
                    .min_w(*sizes::TREEVIEW_INDENT)
                    .border_l_1()
                    .border_color(*colors::BORDER)
            }))
            .child(String::from(self.element.clone()))
            .id("treeview-item")
            .on_mouse_up(MouseButton::Left, |_, _| {})
            .on_hover(cx.listener(|this, hover, cx| {
                this.hover = *hover;
                cx.notify();
            }))
            .on_click(cx.listener(|this, event: &ClickEvent, cx| {
                if event.down.button == MouseButton::Left {
                    cx.update_model(&this.active_element, |active_element, cx| {
                        *active_element = this.element.id();
                        cx.notify();
                    })
                }
            }))
            .when(self.hover, |this| this.bg(*colors::LIST_ITEM_HOVER))
            .when(self.active, |this| this.bg(*colors::LIST_ITEM_ACTIVE))
    }
}
