#[derive(Clone)]
pub struct ElementProperty {
    pub name: String,
    pub content: ElementPropertyType,
}

impl ElementProperty {
    pub fn new_float(name: &str, min: Option<f32>, max: Option<f32>, default: f32) -> Self {
        Self {
            name: name.to_string(),
            content: ElementPropertyType::Float(FloatProperty {
                min,
                max,
                value: default,
            }),
        }
    }

    pub fn new_text(name: &str, default: &str) -> ElementProperty {
        Self {
            name: name.to_string(),
            content: ElementPropertyType::Text(default.to_string()),
        }
    }
}

#[derive(Clone)]
pub enum ElementPropertyType {
    Float(FloatProperty),
    Text(String),
}

#[derive(Clone)]
pub struct FloatProperty {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub value: f32,
}

impl From<ElementPropertyType> for FloatProperty {
    fn from(value: ElementPropertyType) -> Self {
        if let ElementPropertyType::Float(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}

impl From<ElementPropertyType> for String {
    fn from(value: ElementPropertyType) -> Self {
        if let ElementPropertyType::Text(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}
