use super::preview::panel::PreviewPanel;
use super::properties_panel::panel::PropertiesPanel;
use super::treeview::panel::TreeviewPanel;
use crate::appearance::colors;
use crate::appearance::sizes;
use crate::component::Component;
use gpui::*;

pub struct MainView {
    treeview_panel: View<TreeviewPanel>,
    preview_panel: View<PreviewPanel>,
    properties_panel: View<PropertiesPanel>,
}

impl MainView {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let component = Component::from_file("component.xml")
                .unwrap()
                .into_model(cx);
            Component::watch_file("component.xml", component.clone(), cx);

            let treeview_panel = TreeviewPanel::new(component.clone(), cx);
            let preview_panel = PreviewPanel::new(component, cx);
            let properties_panel = PropertiesPanel::new(cx);

            Self {
                treeview_panel,
                preview_panel,
                properties_panel,
            }
        })
    }
}

impl Render for MainView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
    }
}
