use gpui::*;
use prelude::FluentBuilder;

use super::context_menu::ContextMenuGlobal;
use super::preview::panel::PreviewPanel;
use super::properties_panel::panel::PropertiesPanel;
use super::treeview::panel::TreeviewPanel;
use crate::appearance::{colors, sizes};
use crate::component::div::DivComponent;
use crate::component::Component;

pub struct MainView {
    treeview_panel: View<TreeviewPanel>,
    preview_panel: View<PreviewPanel>,
    properties_panel: View<PropertiesPanel>,
}

impl MainView {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let root_component = Component::from(DivComponent::new(cx));
            let selected_id = cx.new_model(|_| None);

            let treeview_panel = TreeviewPanel::new(&root_component, &selected_id, cx);
            let preview_panel = PreviewPanel::new(&root_component, &selected_id, cx);
            let properties_panel = PropertiesPanel::new(&root_component, &selected_id, cx);

            ContextMenuGlobal::init(cx);

            Self {
                treeview_panel,
                preview_panel,
                properties_panel,
            }
        })
    }
}

impl Render for MainView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(*colors::BG)
            .text_color(*colors::TEXT)
            .text_size(*sizes::TEXT_SIZE)
            .font_family("Sans")
            .child(self.treeview_panel.clone())
            .child(self.preview_panel.clone())
            .child(self.properties_panel.clone())
            .on_mouse_down(MouseButton::Left, |_event, cx| {
                ContextMenuGlobal::hide(cx);
            })
            .on_mouse_down(MouseButton::Middle, |_event, cx| {
                ContextMenuGlobal::hide(cx);
            })
            .on_mouse_down(MouseButton::Right, |_event, cx| {
                ContextMenuGlobal::hide(cx);
            })
            .when_some(
                cx.global::<ContextMenuGlobal>().view.clone(),
                |this, context_menu| this.child(context_menu),
            )
    }
}
