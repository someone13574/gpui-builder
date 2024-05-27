use component_preview::ComponentPreviewView;
use gpui::*;
use hierarchy::HierarchyDiv;

mod colors;
mod component_preview;
mod hierarchy;

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

            let hierarchy_model = HierarchyDiv::watch("hierarchy.xml", cx);
            cx.new_view(|_cx| ComponentPreviewView { hierarchy_model })
        });
    });
}
