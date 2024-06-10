use std::any::Any;

use gpui::CursorStyle;

use super::EnumProperty;

pub fn cursor_enum_property() -> EnumProperty {
    EnumProperty {
        value: "Arrow".to_string(),
        valid: vec![
            "Arrow",
            "IBeam",
            "Crosshair",
            "Closed Hand",
            "Open Hand",
            "Resize Left",
            "Resize Right",
            "Resize Left Right",
            "Resize Up",
            "Resize Down",
            "Resize Up Down",
            "Resize Column",
            "Resize Row",
            "Disappearing Item",
            "Vertical IBeam",
            "Not Allowed",
            "Drag Link",
            "Drag Copy",
            "Context Menu",
        ]
        .into_iter()
        .map(|string| string.to_string())
        .collect(),
        to_enum: text_to_enum,
    }
}

fn text_to_enum(text: &str) -> Box<dyn Any> {
    Box::new(match text {
        "Arrow" => CursorStyle::Arrow,
        "IBeam" => CursorStyle::IBeam,
        "Crosshair" => CursorStyle::Crosshair,
        "Closed Hand" => CursorStyle::ClosedHand,
        "Open Hand" => CursorStyle::OpenHand,
        "Pointing Hand" => CursorStyle::PointingHand,
        "Resize Left" => CursorStyle::ResizeLeft,
        "Resize Right" => CursorStyle::ResizeRight,
        "Resize Left Right" => CursorStyle::ResizeLeftRight,
        "Resize Up" => CursorStyle::ResizeUp,
        "Resize Down" => CursorStyle::ResizeDown,
        "Resize Up Down" => CursorStyle::ResizeUpDown,
        "Resize Column" => CursorStyle::ResizeColumn,
        "Resize Row" => CursorStyle::ResizeRow,
        "Disappearing Item" => CursorStyle::DisappearingItem,
        "Vertical IBeam" => CursorStyle::IBeamCursorForVerticalLayout,
        "Not Allowed" => CursorStyle::OperationNotAllowed,
        "Drag Link" => CursorStyle::DragLink,
        "Drag Copy" => CursorStyle::DragCopy,
        "Context Menu" => CursorStyle::ContextualMenu,
        _ => unreachable!("Invalid option {text}"),
    })
}
