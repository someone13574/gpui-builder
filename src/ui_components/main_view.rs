use std::path::Path;

use gpui::*;

use super::preview::Preview;
use super::preview::PreviewState;
use super::treeview::TreeviewPanel;
use crate::appearance::colors;

pub struct MainView {
    state: MainViewState,
}

impl MainView {
    pub fn new(state: MainViewState, cx: &mut WindowContext) -> View<Self> {
        let this = Self { state };
        cx.new_view(|_| this)
    }
}

impl Render for MainView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .flex_grow()
            .items_center()
            .bg(*colors::BG)
            .child(TreeviewPanel::new(self.state.preview_state.clone(), cx))
            .child(Preview::new(self.state.preview_state.clone(), cx))
    }
}

#[derive(Clone)]
pub struct MainViewState {
    preview_state: PreviewState,
}

impl MainViewState {
    pub fn new<P: AsRef<Path>>(component_path: P, cx: &mut AppContext) -> Self {
        Self {
            preview_state: PreviewState::new(component_path, cx),
        }
    }
}
