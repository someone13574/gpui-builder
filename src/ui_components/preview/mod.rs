use crate::appearance::colors;
use crate::hierarchy::{Component, Element};
use div::DivPreview;
use gpui::*;
use notify::Watcher;
use std::path::Path;

mod div;

pub struct Preview {
    pub component_model: Model<Component>,
}

impl Preview {
    pub fn new<V: Render + 'static, P: AsRef<Path>>(
        cx: &mut ViewContext<V>,
        component_path: P,
    ) -> View<Preview> {
        let component_model = watched_component(cx, component_path);
        cx.new_view(|_| Self { component_model })
    }
}

impl Render for Preview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.component_model, |root, _| root.clone())
            .element;

        let this = div()
            .bg(*colors::BG)
            .size_full()
            .justify_center()
            .items_center();

        match root_element {
            Element::Div(div) => this.child(DivPreview::from(div)),
            Element::Text(text) => this.child(text),
        }
    }
}

fn watched_component<P: AsRef<Path>>(
    cx: &mut WindowContext,
    component_path: P,
) -> Model<Component> {
    let component_path = component_path.as_ref().to_path_buf();
    let (tx, rx) = async_channel::unbounded();
    let model = cx.new_model(|_| Component::from_file(&component_path).unwrap());

    let component_path_clone = component_path.clone();
    let mut watcher = notify::recommended_watcher(move |result| match result {
        Ok(_) => {
            if let Ok(component) = Component::from_file(&component_path_clone) {
                tx.send_blocking(component).unwrap();
            }
        }
        Err(_) => todo!(),
    })
    .unwrap();

    let model_clone = model.clone();
    cx.spawn(|mut cx| async move {
        watcher
            .watch(&component_path, notify::RecursiveMode::NonRecursive)
            .unwrap();

        while let Ok(new_component) = rx.recv().await {
            cx.update_model(&model_clone, |component, _| {
                *component = new_component;
            })
            .unwrap();
        }
    })
    .detach();
    model
}
