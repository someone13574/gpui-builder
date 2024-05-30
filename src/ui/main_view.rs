use gpui::*;

use crate::{appearance::colors, component::Component};

use super::treeview::panel::TreeviewPanel;

pub struct MainView {
    treeview_panel: View<TreeviewPanel>,
}

impl MainView {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|cx| {
            let component = Component::from_file("component.xml")
                .unwrap()
                .into_model(cx);
            Component::watch_file("component.xml", component.clone(), cx);

            let treeview_panel = TreeviewPanel::new(component, cx);

            Self { treeview_panel }
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
            .font_family("Sans")
            .child(self.treeview_panel.clone())
    }
}
