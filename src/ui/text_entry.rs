use std::ops::Range;

use gpui::*;

pub struct TextModel {
    pub text: String,
    pub selection: Range<usize>,
}

impl TextModel {
    pub fn new(text: String, cx: &mut AppContext) -> Model<Self> {
        let end_of_text = text.len();
        cx.new_model(|_cx| Self {
            text,
            selection: end_of_text..end_of_text,
        })
    }
}

pub struct TextEntry {
    model: Model<TextModel>,
    focus_handle: FocusHandle,
    filter_input: Box<dyn Fn(String) -> bool>,
}

impl TextEntry {
    pub fn new<V: 'static>(
        model: Model<TextModel>,
        filter_input: Box<dyn Fn(String) -> bool>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&model, |_, _, cx| cx.notify()).detach();
            Self {
                model,
                filter_input,
                focus_handle: cx.focus_handle(),
            }
        })
    }
}

impl Render for TextEntry {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.focus(&self.focus_handle);

        let text = self.model.read(cx).text.clone();
        div()
            .track_focus(&self.focus_handle)
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

                            if (this.filter_input)(key.clone()) {
                                model.text.push_str(&key);
                                cx.notify();
                            }
                        }
                        _ => (),
                    }
                });
            }))
            .child(InteractiveText::new("entry", StyledText::new(text)))
    }
}
