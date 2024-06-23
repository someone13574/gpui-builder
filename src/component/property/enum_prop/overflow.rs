use std::any::Any;

use gpui::Overflow;

use super::EnumProperty;

pub enum OverflowEnumDirection {
    X,
    Y,
}

pub fn overflow_enum_property(direction: OverflowEnumDirection) -> EnumProperty {
    EnumProperty {
        value: "Visible".to_string(),
        valid: vec!["Visible", "Hidden", "Scroll"]
            .into_iter()
            .map(|string| string.to_string())
            .collect(),
        extra_data: match direction {
            OverflowEnumDirection::X => "x",
            OverflowEnumDirection::Y => "y",
        }
        .to_string(),
        to_enum: text_to_enum,
        to_code,
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

fn to_code(text: &str, extra_data: &str) -> String {
    match (text, extra_data) {
        ("Visible", "x" | "y") => "",
        ("Hidden", "x") => ".overflow_x_hidden()",
        ("Scroll", "x") => ".overflow_x_scroll()",
        ("Hidden", "y") => ".overflow_y_hidden()",
        ("Scroll", "y") => ".overflow_y_scroll()",
        _ => unreachable!(),
    }
    .to_string()
}
