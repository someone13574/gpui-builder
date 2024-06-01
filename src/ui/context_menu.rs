use gpui::*;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::main_view::MainView;
use crate::appearance::colors;

pub struct ContextMenuGlobal {
    pub view: Option<View<ContextMenu>>,
}

impl ContextMenuGlobal {
    pub fn init(cx: &mut ViewContext<MainView>) {
        if cx.has_global::<Self>() {
            panic!("Context menu global already initialized");
        }

        cx.observe_global::<Self>(|_, cx| {
            cx.notify();
        })
        .detach();
        cx.set_global(Self { view: None });
    }

    pub fn activate(
        position: Point<Pixels>,
        actions: Vec<Vec<ContextMenuAction>>,
        cx: &mut WindowContext,
    ) {
        let this = Self {
            view: Some(cx.new_view(|cx| {
                ContextMenu {
                    position,
                    item_groups: actions
                        .into_iter()
                        .map(|action_group| {
                            action_group
                                .into_iter()
                                .map(|action| ContextMenuItem::new(action, cx))
                                .collect()
                        })
                        .collect(),
                }
            })),
        };
        cx.set_global(this);
    }

    pub fn hide(cx: &mut AppContext) {
        cx.set_global(Self { view: None })
    }
}

impl Global for ContextMenuGlobal {}

pub struct ContextMenu {
    position: Point<Pixels>,
    item_groups: Vec<Vec<View<ContextMenuItem>>>,
}

impl Render for ContextMenu {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        anchored().position(self.position).child(
            deferred(
                div()
                    .occlude()
                    .bg(*colors::CONTEXT_MENU_BG)
                    .border_1()
                    .border_color(*colors::BORDER)
                    .rounded(px(8.0))
                    .children(
                        self.item_groups
                            .iter()
                            .enumerate()
                            .map(|(idx, item_group)| {
                                div()
                                    .flex()
                                    .flex_col()
                                    .px_2()
                                    .py_1()
                                    .gap_1()
                                    .when(idx != 0, |this| {
                                        this.border_t_1().border_color(*colors::BORDER)
                                    })
                                    .children(item_group.clone())
                            }),
                    ),
            )
            .priority(1),
        )
    }
}

pub struct ContextMenuAction {
    text: String,
}

impl ContextMenuAction {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

struct ContextMenuItem {
    text: String,

    id: Uuid,
    hover: bool,
}

impl ContextMenuItem {
    pub fn new(action: ContextMenuAction, cx: &mut ViewContext<ContextMenu>) -> View<Self> {
        cx.new_view(|_| {
            Self {
                text: action.text,
                id: Uuid::new_v4(),
                hover: false,
            }
        })
    }
}

impl Render for ContextMenuItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(self.text.clone())
            .px_1()
            .rounded(px(4.0))
            .when(self.hover, |this| this.bg(*colors::LIST_ITEM_HOVER))
            .on_mouse_down(MouseButton::Left, |_, _| {})
            .id(self.id)
            .on_hover(cx.listener(|this, hover, cx| {
                this.hover = *hover;
                cx.notify();
            }))
            .on_click(|_, _| println!("Click!"))
    }
}
