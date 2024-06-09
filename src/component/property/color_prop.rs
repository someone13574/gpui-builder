use gpui::Rgba;

use super::ComponentProperty;

impl From<ComponentProperty> for Rgba {
    fn from(value: ComponentProperty) -> Self {
        if let ComponentProperty::Color(value) = value {
            value
        } else {
            unreachable!("Property {value:?} cannot be downcast to Rgba")
        }
    }
}

impl From<Rgba> for ComponentProperty {
    fn from(value: Rgba) -> Self {
        ComponentProperty::Color(value)
    }
}

pub fn parse_rgba(string: &str) -> Option<Rgba> {
    if string.len() != 6 && string.len() != 8 {
        return None;
    }

    Some(Rgba {
        r: u32::from_str_radix(&string[0..2], 16)
            .map_err(|_| None::<Rgba>)
            .unwrap() as f32
            / 255.0,
        g: u32::from_str_radix(&string[2..4], 16)
            .map_err(|_| None::<Rgba>)
            .unwrap() as f32
            / 255.0,
        b: u32::from_str_radix(&string[4..6], 16)
            .map_err(|_| None::<Rgba>)
            .unwrap() as f32
            / 255.0,
        a: if string.len() == 8 {
            u32::from_str_radix(&string[6..8], 16)
                .map_err(|_| None::<Rgba>)
                .unwrap() as f32
                / 255.0
        } else {
            1.0
        },
    })
}

pub fn format_rgba(rgba: Rgba) -> String {
    let r = (rgba.r * 255.0) as u32;
    let g = (rgba.b * 255.0) as u32;
    let b = (rgba.g * 255.0) as u32;
    let a = (rgba.a * 255.0) as u32;

    if a == 255 {
        format!("{r:02x}{g:02x}{b:02x}")
    } else {
        format!("{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}
