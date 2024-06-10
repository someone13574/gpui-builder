use gpui::ViewContext;

use super::item::TreeviewItem;
use crate::component::div::DivComponent;
use crate::component::text::TextComponent;
use crate::component::Component;
use crate::ui::context_menu::ContextMenuAction;

pub fn item_context_actions(
    item: &TreeviewItem,
    cx: &mut ViewContext<TreeviewItem>,
) -> Vec<Vec<ContextMenuAction>> {
    let mut actions = Vec::new();

    match item.component {
        Component::Div(_) => {
            actions.push(vec![
                add_child_action("New `div` child", DivComponent::new(cx).into(), cx),
                add_child_action("New `text` child", TextComponent::new(cx).into(), cx),
            ]);
        }
        Component::Text(_) => {}
    }

    actions.push(vec![move_up_action(cx), move_down_action(cx)]);
    actions.push(vec![delete_action(cx)]);

    actions
}

pub fn add_child_action(
    title: &str,
    child: Component,
    cx: &mut ViewContext<TreeviewItem>,
) -> ContextMenuAction {
    ContextMenuAction::new(
        title.to_string(),
        cx.listener(move |this, _, cx| {
            let component: DivComponent = this.component.clone().into();
            let child = child.clone();
            let new_id = child.id();

            component.add_child(child, cx);
            this.active_id.update(cx, |active_id, cx| {
                *active_id = Some(new_id);
                cx.notify();
            });
        }),
    )
}

pub fn move_up_action(cx: &mut ViewContext<TreeviewItem>) -> ContextMenuAction {
    ContextMenuAction::new(
        "Move up".to_string(),
        cx.listener(|this, _, cx| {
            let parent: DivComponent = (*this.component.parent().unwrap()).into();
            let children = parent.children.read(cx).clone();
            let component_id = this.component.id();

            if let Some(pos) = children.iter().position(|child| child.id() == component_id) {
                if pos != 0 {
                    parent.children.update(cx, |children, cx| {
                        children.swap(pos, pos - 1);
                        cx.notify();
                    });
                } else if let Some(grandparent) = parent.parent {
                    let grandparent: DivComponent = (*grandparent).into();
                    let component = parent.children.update(cx, |children, cx| {
                        let component = children.remove(pos);
                        cx.notify();
                        component
                    });
                    grandparent.children.update(cx, |grandchildren, cx| {
                        let parent_pos = grandchildren
                            .iter()
                            .position(|child| child.id() == parent.id)
                            .unwrap();
                        grandchildren.insert(
                            parent_pos,
                            component.with_parent(grandparent.clone().into()),
                        );
                        cx.notify();
                    });
                }
            }

            this.active_id.update(cx, |active_id, cx| {
                *active_id = Some(component_id);
                cx.notify();
            });
        }),
    )
}

pub fn move_down_action(cx: &mut ViewContext<TreeviewItem>) -> ContextMenuAction {
    ContextMenuAction::new(
        "Move down".to_string(),
        cx.listener(|this, _, cx| {
            let parent: DivComponent = (*this.component.parent().unwrap()).into();
            let children = parent.children.read(cx).clone();
            let component_id = this.component.id();

            if let Some(pos) = children.iter().position(|child| child.id() == component_id) {
                if pos != children.len() - 1 {
                    parent.children.update(cx, |children, cx| {
                        children.swap(pos, pos + 1);
                        cx.notify();
                    });
                } else if let Some(grandparent) = parent.parent {
                    let grandparent: DivComponent = (*grandparent).into();
                    let component = parent.children.update(cx, |children, cx| {
                        let component = children.remove(pos);
                        cx.notify();
                        component
                    });
                    grandparent.children.update(cx, |grandchildren, cx| {
                        let parent_pos = grandchildren
                            .iter()
                            .position(|child| child.id() == parent.id)
                            .unwrap();
                        grandchildren.insert(
                            parent_pos + 1,
                            component.with_parent(grandparent.clone().into()),
                        );
                        cx.notify();
                    });
                }
            }

            this.active_id.update(cx, |active_id, cx| {
                *active_id = Some(component_id);
                cx.notify();
            });
        }),
    )
}

fn delete_action(cx: &mut ViewContext<TreeviewItem>) -> ContextMenuAction {
    ContextMenuAction::new(
        "Delete".to_string(),
        cx.listener(|this, _, cx| {
            let parent: DivComponent = (*this.component.parent().unwrap()).into();
            parent.children.update(cx, |children, cx| {
                *children = children
                    .iter()
                    .filter(|child| child.id() != this.component.id())
                    .cloned()
                    .collect();
                cx.notify();
            });
            this.active_id.update(cx, |active_id, cx| {
                *active_id = Some(this.component.id());
                cx.notify();
            });
        }),
    )
}
