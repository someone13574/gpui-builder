use gpui::*;

use super::button::Button;
use crate::appearance::colors;
use crate::component::save_load::from_serde::FromSerde;
use crate::component::save_load::to_serde::ToSerde;
use crate::component::save_load::SerdeComponent;
use crate::component::Component;

#[derive(IntoElement, Clone)]
pub struct TitleBar {
    save_button: View<Button>,
    open_button: View<Button>,
}

impl TitleBar {
    pub fn new(root_component: &Model<Component>, cx: &mut WindowContext) -> Self {
        let root_component_clone = root_component.clone();
        let root_component_clone_2 = root_component.clone();

        Self {
            save_button: Button::new(
                "Save",
                move |_, cx| {
                    let component = cx.read_model(&root_component_clone, |component, cx| {
                        component.to_serde(cx)
                    });
                    cx.spawn(|_| async move {
                        if let Some(file) = rfd::AsyncFileDialog::new()
                            .set_file_name("component.json")
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
            open_button: Button::new(
                "Open",
                move |_, cx| {
                    let root_component_clone = root_component_clone_2.clone();

                    cx.spawn(|mut cx| async move {
                        if let Some(file) = rfd::AsyncFileDialog::new()
                            .add_filter("json", &["json"])
                            .pick_file()
                            .await
                        {
                            let contents = String::from_utf8(file.read().await).unwrap();
                            let component: SerdeComponent =
                                serde_json::from_str(&contents).unwrap();
                            cx.update_model(&root_component_clone, |root_component, cx| {
                                *root_component = Component::from_serde(component, cx);
                                cx.notify();
                            })
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
            .gap_x_2()
            .p_1p5()
            .w_full()
            .bg(*colors::BG)
            .border_b_1()
            .border_color(*colors::BORDER)
            .child(self.save_button)
            .child(self.open_button)
    }
}
