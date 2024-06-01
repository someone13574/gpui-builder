use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use crate::appearance::{colors, sizes};
use crate::component::element::ComponentElement;
use crate::ui::context_menu::{ContextMenuAction, ContextMenuGlobal};

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
                this.active = Some(this.element.id(cx))
                    == cx.read_model(&active_element, |active_element, _| *active_element);
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
            .child(format_element_name(&self.element, cx))
            .id("treeview-item")
            .on_mouse_up(
                MouseButton::Right,
                cx.listener(|_, event: &MouseUpEvent, cx| {
                    ContextMenuGlobal::activate(
                        event.position,
                        vec![
                            ContextMenuAction::new("Example action 1".to_string()),
                            ContextMenuAction::new("Example action 2".to_string()),
                            ContextMenuAction::new("Example action 3".to_string()),
                        ],
                        cx,
                    )
                }),
            )
            .on_hover(cx.listener(|this, hover, cx| {
                this.hover = *hover;
                cx.notify();
            }))
            .on_click(cx.listener(|this, event: &ClickEvent, cx| {
                if event.down.button == MouseButton::Left {
                    cx.update_model(&this.active_element, |active_element, cx| {
                        *active_element = Some(this.element.id(cx));
                        cx.notify();
                    })
                }
            }))
            .when(self.hover, |this| this.bg(*colors::LIST_ITEM_HOVER))
            .when(self.active, |this| this.bg(*colors::LIST_ITEM_ACTIVE))
    }
}

fn format_element_name(element: &ComponentElement, cx: &mut AppContext) -> String {
    match element {
        ComponentElement::Div(_) => "div:".to_string(),
        ComponentElement::Text(element) => {
            format!(
                "\"{}\"",
                String::from(element.read(cx).properties.get("text").unwrap().clone())
            )
        }
    }
}
