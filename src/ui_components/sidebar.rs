use gpui::*;
use prelude::FluentBuilder;

use crate::appearance::{colors, sizes};

#[derive(PartialEq, Eq)]
pub enum SidebarSide {
    Left,
    Right,
}

#[derive(IntoElement)]
pub struct Sidebar {
    pub side: SidebarSide,
}

impl RenderOnce for Sidebar {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .bg(*colors::SIDEBAR_BG)
            .border_color(*colors::BORDER)
            .h_full()
            .min_w(*sizes::SIDEBAR_WIDTH)
            .when(self.side == SidebarSide::Left, |this| {
                this.border_r(*sizes::BORDER_WIDTH)
            })
            .when(self.side == SidebarSide::Right, |this| {
                this.border_l(*sizes::BORDER_WIDTH)
            })
    }
}
