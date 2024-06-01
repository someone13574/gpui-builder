use gpui::Rgba;

#[derive(Clone)]
pub enum ElementProperty {
    Float(FloatProperty),
    Text(String),
    Color(Rgba),
}

impl From<FloatProperty> for ElementProperty {
    fn from(value: FloatProperty) -> Self {
        Self::Float(value)
    }
}

impl From<String> for ElementProperty {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Rgba> for ElementProperty {
    fn from(value: Rgba) -> Self {
        Self::Color(value)
    }
}

#[derive(Clone)]
pub struct FloatProperty {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub value: f32,
}

impl From<ElementProperty> for FloatProperty {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Float(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}

impl From<ElementProperty> for String {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Text(value) = value {
            value
        } else {
            unreachable!()
        }
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

impl From<ElementProperty> for Rgba {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Color(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}
