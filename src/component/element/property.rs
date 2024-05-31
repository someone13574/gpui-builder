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
}

#[derive(Clone)]
pub enum ElementPropertyType {
    Float(FloatProperty),
    _String,
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
