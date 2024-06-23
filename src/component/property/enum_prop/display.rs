use std::any::Any;

use gpui::Display;

use super::EnumProperty;

pub fn display_enum_property() -> EnumProperty {
    EnumProperty {
        value: "Block".to_string(),
        valid: vec!["Block", "Flex"]
            .into_iter()
            .map(|string| string.to_string())
            .collect(),
        extra_data: String::new(),
        to_enum: text_to_enum,
        to_code,
    }
}

fn text_to_enum(text: &str) -> Box<dyn Any> {
    Box::new(match text {
        "Block" => Display::Block,
        "Flex" => Display::Flex,
        _ => unreachable!("Invalid option {text}"),
    })
}

fn to_code(text: &str, _extra_data: &str) -> String {
    match text {
        "Block" => ".block()",
        "Flex" => ".flex()",
        _ => unreachable!(),
    }
    .to_string()
}
