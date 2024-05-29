use gpui::*;

use crate::{
    appearance::{colors, sizes},
    ui_components::preview::PreviewState,
};

use super::item::get_tree;

pub struct TreeviewPanel {
    pub state: PreviewState,
}

impl TreeviewPanel {
    pub fn into_view<V: 'static>(self, cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|_| self)
    }
}

impl Render for TreeviewPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.state.component, |root, _| root.clone())
            .element;

        let items = get_tree(&root_element, 0)
            .into_iter()
            .map(|item| item.into_view(cx))
            .collect::<Vec<_>>();

        div()
            .h_full()
            .min_w(*sizes::SIDEBAR_WIDTH)
            .bg(*colors::SIDEBAR_BG)
            .p_4()
            .border_r(*sizes::BORDER_WIDTH)
            .border_color(*colors::BORDER)
            .text_color(*colors::TEXT)
            .font_family("Sans")
            .children(items)
    }
}
