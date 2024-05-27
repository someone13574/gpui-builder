use gpui::{Context, WindowContext};
use notify::Watcher;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "div")]
pub struct HierarchyDiv {
    pub text: Option<String>,
    #[serde(rename = "div", default)]
    pub children: Vec<HierarchyDiv>,
}

impl HierarchyDiv {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<HierarchyDiv> {
        let src = fs::read_to_string(path).unwrap();
        if let Ok(div) = serde_xml_rs::from_str(&src) {
            Some(div)
        } else {
            None
        }
    }

    pub fn watch<P: AsRef<Path>>(path: P, cx: &mut WindowContext) -> gpui::Model<HierarchyDiv> {
        let path = path.as_ref().to_path_buf();
        let (tx, rx) = async_channel::unbounded();

        let path_clone = path.clone();
        let mut watcher = notify::recommended_watcher(move |res| match res {
            Ok(_) => {
                if let Some(div) = HierarchyDiv::from_file(&path_clone) {
                    tx.send_blocking(div).unwrap();
                }
            }
            Err(e) => println!("watcher error: {:?}", e),
        })
        .unwrap();

        let model = cx.new_model(|_| HierarchyDiv::from_file(&path).unwrap());
        let model_clone = model.clone();

        cx.spawn(|mut cx| async move {
            watcher
                .watch(&path, notify::RecursiveMode::NonRecursive)
                .unwrap();

            while let Ok(new_hierarchy) = rx.recv().await {
                cx.update_model(&model_clone, |hierarchy, _| {
                    *hierarchy = new_hierarchy;
                })
                .unwrap();
            }
        })
        .detach();
        model
    }
}
