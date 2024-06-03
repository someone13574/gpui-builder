use gpui::prelude::*;
use gpui::*;

pub struct CheckBox {
    model: Model<bool>,

    checked: bool,
    hovered: bool,
}

impl CheckBox {
    pub fn new<V: 'static>(model: Model<bool>, cx: &mut ViewContext<V>) -> View<Self> {
        cx.new_view(|cx| {
            cx.observe(&model, |this: &mut Self, _, cx| {
                this.checked = !this.checked;
                cx.notify();
            })
            .detach();

            let checked = *model.read(cx);

            Self {
                model,
                checked,
                hovered: false,
            }
        })
    }
}

impl Render for CheckBox {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_5()
            .rounded(px(4.0))
            .border_1()
            .border_color(rgb(0x505050))
            .when(self.hovered, |this| this.bg(rgba(0xffffff04)))
            .when(self.checked, |this| {
                this.bg(rgb(0xff0000)).p(px(2.0)).child(
                    svg()
                        .path("assets/icons/check.svg") // TODO: Fix this
                        .text_color(white())
                        .size_full(),
                )
            })
            .id("checkbox")
            .on_click(cx.listener(|this, _, cx| {
                this.model.update(cx, |checkbox, cx| {
                    *checkbox = !*checkbox;
                    cx.notify();
                })
            }))
            .on_hover(cx.listener(|this, hovered, cx| {
                this.hovered = *hovered;
                cx.notify();
            }))
    }
}
