use element::ComponentElement;
use gpui::AppContext;

pub mod element;

#[derive(Clone)]
pub struct Component {
    pub root: Option<ComponentElement>,
}

impl Component {
    pub fn element_list(&self, cx: &mut AppContext) -> Vec<(ComponentElement, u32)> {
        if let Some(root) = &self.root {
            root.element_list(0, cx)
        } else {
            Vec::new()
        }
    }
}
