use std::any::Any;

use gpui::Overflow;

use super::EnumProperty;

pub fn overflow_enum_property() -> EnumProperty {
    EnumProperty {
        value: "Visible".to_string(),
        valid: vec!["Visible", "Hidden", "Scroll"]
            .into_iter()
            .map(|string| string.to_string())
            .collect(),
        to_enum: text_to_enum,
    }
}

fn text_to_enum(text: &str) -> Box<dyn Any> {
    Box::new(match text {
        "Visible" => Overflow::Visible,
        "Hidden" => Overflow::Hidden,
        "Scroll" => Overflow::Scroll,
        _ => unreachable!("Invalid option {text}"),
    })
}
