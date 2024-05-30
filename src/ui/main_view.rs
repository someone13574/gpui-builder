use super::preview::panel::PreviewPanel;
use super::treeview::panel::TreeviewPanel;
use crate::appearance::colors;
use crate::appearance::sizes;
use crate::component::Component;
use gpui::*;

pub struct MainView {
    treeview_panel: View<TreeviewPanel>,
    preview_panel: View<PreviewPanel>,
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

            Self {
                treeview_panel,
                preview_panel,
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
    }
}
