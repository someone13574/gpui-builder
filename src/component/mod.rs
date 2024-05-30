use element::ComponentElement;
use gpui::{AppContext, Context, Model};
use notify::Watcher;
use serde::Deserialize;
use std::{fs, path::Path};

pub mod element;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "component")]
pub struct Component {
    #[serde(rename = "$value")]
    pub root: ComponentElement,
}

impl Component {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, serde_xml_rs::Error> {
        let file_content = fs::read_to_string(path).unwrap();
        serde_xml_rs::from_str(&file_content)
    }

    pub fn assign_element_ids(mut self) -> Self {
        self.root.assign_id_recursive();
        self
    }

    pub fn into_model(self, cx: &mut AppContext) -> Model<Self> {
        cx.new_model(|_| self)
    }

    pub fn watch_file<P: AsRef<Path>>(
        path: P,
        component_model: Model<Component>,
        cx: &mut AppContext,
    ) {
        let path = path.as_ref().to_path_buf();
        let (tx, rx) = async_channel::unbounded();

        let mut watcher = notify::recommended_watcher({
            let path = path.clone();
            move |result| match result {
                Ok(_) => {
                    if let Ok(component) = Component::from_file(&path) {
                        tx.send_blocking(component).unwrap();
                    }
                }
                err => {
                    err.expect("watcher error");
                }
            }
        })
        .unwrap();

        cx.spawn(|mut cx| async move {
            watcher
                .watch(&path, notify::RecursiveMode::NonRecursive)
                .unwrap();

            while let Ok(new) = rx.recv().await {
                cx.update_model(&component_model, |component, cx| {
                    *component = new;
                    cx.notify();
                })
                .unwrap();
            }
        })
        .detach();
    }
}
