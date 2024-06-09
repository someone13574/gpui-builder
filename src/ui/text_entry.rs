use gpui::*;
use uuid::Uuid;

use crate::appearance::colors;

pub struct TextModel {
    pub text: String,
}

impl TextModel {
    pub fn new(text: String, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_cx| Self { text })
    }
}

pub struct TextEntry {
    pub id: Uuid,
    model: Model<TextModel>,
    focus_handle: FocusHandle,
    filter_input: fn(char) -> bool,
}

impl TextEntry {
    pub fn new<V: 'static>(
        model: Model<TextModel>,
        filter_input: fn(char) -> bool,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&model, |_, _, cx| cx.notify()).detach();
            Self {
                id: Uuid::new_v4(),
                model,
                filter_input,
                focus_handle: cx.focus_handle(),
            }
        })
    }
}

impl Render for TextEntry {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.model.read(cx).text.clone();
        div()
            .w_full()
            .border_color(*colors::BORDER)
            .border_1()
            .rounded(px(8.0))
            .px_1()
            .cursor_text()
            .id(self.id)
            .track_focus(&self.focus_handle)
            .on_click(cx.listener(|this, _, cx| {
                cx.focus(&this.focus_handle);
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, cx| {
                cx.update_model(&this.model, |model, cx| {
                    match event.keystroke.key.as_str() {
                        "space" => {
                            model.text.push(' ');
                            cx.notify();
                        }
                        "backspace" => {
                            model.text.pop();
                            cx.notify();
                        }
                        key if key.len() == 1 => {
                            let key = if event.keystroke.modifiers.shift {
                                key.to_uppercase()
                            } else {
                                key.to_string()
                            };

                            if (this.filter_input)(key.chars().next().unwrap()) {
                                model.text.push_str(&key);
                                cx.notify();
                            }
                        }
                        _ => (),
                    }
                });
            }))
            .child(InteractiveText::new("entry", StyledText::new(text)))
            .focus(|this| this.bg(rgba(0xffffff04)))
    }
}
