use gpui::*;
use hierarchy::HierarchyDiv;
use std::sync::Arc;

mod colors;
mod hierarchy;

struct RootView;

impl Render for RootView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().bg(*colors::BACKGROUND)
    }
}

fn main() {
    HierarchyDiv::watch("./hierarchy.xml", Arc::new(|div| println!("{:#?}", div)));

    App::new().run(|cx: &mut AppContext| {
        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI Builder".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            ..Default::default()
        };

        cx.open_window(options, |cx| {
            cx.on_window_should_close(|cx| {
                cx.remove_window();
                true
            });

            cx.new_view(|_cx| RootView {})
        });
    });
}
