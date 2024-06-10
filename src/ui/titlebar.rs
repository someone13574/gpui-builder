use std::fs;

use gpui::*;

use super::button::Button;
use crate::appearance::colors;
use crate::component::save_load::conversion::ToSerde;
use crate::component::Component;

#[derive(IntoElement, Clone)]
pub struct TitleBar {
    save_button: View<Button>,
}

impl TitleBar {
    pub fn new(root_component: &Component, cx: &mut WindowContext) -> Self {
        let root_component = root_component.clone();

        Self {
            save_button: Button::new(
                "Save",
                move |_, cx| {
                    fs::write(
                        "saved.json",
                        serde_json::to_string_pretty(&root_component.to_serde(cx))
                            .expect("Failed to serialize component"),
                    )
                    .expect("Failed to save component file");
                },
                cx,
            ),
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .p_1p5()
            .w_full()
            .bg(*colors::BG)
            .border_b_1()
            .border_color(*colors::BORDER)
            .child(self.save_button)
    }
}
