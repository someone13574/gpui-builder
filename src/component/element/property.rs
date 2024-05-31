#[derive(Clone)]
pub enum ElementProperty {
    Float(FloatProperty),
    Text(String),
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
