use std::borrow::Cow;
use std::fs;

use anyhow::{Context, Result};
use gpui::*;

pub struct Assets {}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>> {
        let data =
            fs::read(path).with_context(|| format!("Failed to read file at path: {}", path))?;
        Ok(Cow::from(data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|entry| {
                Some(SharedString::from(
                    entry.ok()?.path().to_string_lossy().to_string(),
                ))
            })
            .collect::<Vec<_>>())
    }
}
