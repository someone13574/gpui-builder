use gpui::*;
use ui_components::main_view::{MainView, MainViewState};

mod appearance;
mod hierarchy;
mod ui_components;

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

            MainView::new(MainViewState::new("hierarchy.xml", cx), cx)
        });
    });
}
