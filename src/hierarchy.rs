use std::{fs, path::Path, sync::Arc, thread};

use notify::Watcher;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "div")]
pub struct HierarchyDiv {
    text: Option<String>,
    #[serde(rename = "div", default)]
    children: Vec<HierarchyDiv>,
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

    pub fn watch<P: AsRef<Path>>(path: P, f: Arc<dyn Fn(HierarchyDiv) + Sync + Send>) {
        let path = path.as_ref().to_path_buf();
        if let Some(div) = HierarchyDiv::from_file(&path) {
            f(div);
        }

        thread::spawn(move || {
            let path_clone = path.clone();
            let mut watcher = notify::recommended_watcher(move |res| match res {
                Ok(_) => {
                    if let Some(div) = HierarchyDiv::from_file(&path_clone) {
                        f(div);
                    }
                }
                Err(e) => println!("watcher error: {:?}", e),
            })
            .unwrap();

            watcher
                .watch(path.as_path(), notify::RecursiveMode::NonRecursive)
                .unwrap();

            // TODO: Not this
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}
