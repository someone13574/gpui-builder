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
                    let component = root_component.to_serde(cx);
                    cx.spawn(|_| async move {
                        if let Some(file) = rfd::AsyncFileDialog::new()
                            .set_file_name("component.json")
                            .set_directory(".")
                            .add_filter("json", &["json"])
                            .save_file()
                            .await
                        {
                            file.write(
                                &serde_json::to_vec(&component)
                                    .expect("Failed to serialize component"),
                            )
                            .await
                            .unwrap();
                        }
                    })
                    .detach();
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