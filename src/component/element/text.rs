use gpui::{AppContext, Context};
use uuid::Uuid;

use super::ComponentElement;

#[derive(Clone)]
pub struct TextElement {
    pub text: String,
    pub id: Uuid,
}

impl TextElement {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(text: &str, cx: &mut AppContext) -> ComponentElement {
        ComponentElement::Text(cx.new_model(|_| Self {
            text: text.to_string(),
            id: Uuid::new_v4(),
        }))
    }
}
