use crate::appearance::colors;
use crate::hierarchy::{Component, Element};
use div::DivPreview;
use gpui::*;
use notify::Watcher;
use std::path::Path;

mod div;

pub struct Preview {
    state: PreviewState,
}

impl Preview {
    pub fn new<T: 'static>(state: PreviewState, cx: &mut ViewContext<T>) -> View<Self> {
        cx.observe(&state.component, |_, _, cx| cx.notify())
            .detach();
        cx.new_view(|_| Self { state })
    }
}

impl Render for Preview {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let root_element = cx
            .read_model(&self.state.component, |root, _| root.clone())
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

#[derive(Clone)]
pub struct PreviewState {
    component: Model<Component>,
}

impl PreviewState {
    pub fn new<P: AsRef<Path>>(component_path: P, cx: &mut AppContext) -> Self {
        Self {
            component: watched_component(cx, component_path),
        }
    }
}

fn watched_component<P: AsRef<Path>>(cx: &mut AppContext, component_path: P) -> Model<Component> {
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
            cx.update_model(&model_clone, |component, cx| {
                *component = new_component;
                cx.notify();
            })
            .unwrap();
        }
    })
    .detach();
    model
}
