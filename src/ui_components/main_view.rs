use gpui::*;

use super::sidebar::SidebarSide;
use super::{preview::Preview, sidebar::Sidebar};
use crate::appearance::colors;

pub struct MainView;

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
            .child(Preview::new(cx, "hierarchy.xml"))
            .child(Sidebar {
                side: SidebarSide::Right,
            })
    }
}
