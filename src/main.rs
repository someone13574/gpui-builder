use gpui::*;
use ui::main_view::MainView;

mod appearance;
mod component;
mod ui;

fn main() {
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

            MainView::new(cx)
        });
    });
}
