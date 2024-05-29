use gpui::*;

use crate::{
    appearance::{colors, sizes},
    ui_components::preview::PreviewState,
};

use super::item::TreeviewItem;

#[derive(IntoElement)]
pub struct TreeviewPanel {
    state: PreviewState,
}

impl TreeviewPanel {
    pub fn new<T: 'static>(state: PreviewState, _cx: &mut ViewContext<T>) -> Self {
        Self { state }
    }
}

impl RenderOnce for TreeviewPanel {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.state.component, |root, _| root.clone())
            .element;

        div()
            .h_full()
            .min_w(*sizes::SIDEBAR_WIDTH)
            .bg(*colors::SIDEBAR_BG)
            .p_4()
            .border_r(*sizes::BORDER_WIDTH)
            .border_color(*colors::BORDER)
            .child(TreeviewItem {
                element: root_element,
                root: true,
            })
    }
}
