use crate::{appearance::colors, hierarchy::Element};
use gpui::*;

pub fn get_tree(element: &Element, indent: u32) -> Vec<TreeViewItem> {
    let mut flat_tree = vec![TreeViewItem {
        text: element.clone().into(),
        indent,
    }];
    if let Element::Div(div) = element {
        for child in &div.children {
            flat_tree.append(&mut get_tree(child, indent + 1));
        }
    }
    flat_tree
}

#[derive(Default)]
pub struct TreeViewItem {
    pub text: String,
    pub indent: u32,
}

impl TreeViewItem {
    pub fn into_view<V: 'static>(self, cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|_| self)
    }
}

impl Render for TreeViewItem {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .children((0..self.indent).map(|_| {
                div()
                    .min_w(px(32.0))
                    .border_l_1()
                    .border_color(*colors::BORDER)
            }))
            .child(self.text.clone())
    }
}
