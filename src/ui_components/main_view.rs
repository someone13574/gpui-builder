use std::path::Path;

use gpui::*;

use super::preview::PreviewState;
use super::sidebar::SidebarSide;
use super::{preview::Preview, sidebar::Sidebar};
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
            .child(Sidebar {
                side: SidebarSide::Left,
            })
            .child(Preview::new(self.state.preview_state.clone(), cx))
            .child(Sidebar {
                side: SidebarSide::Right,
            })
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
