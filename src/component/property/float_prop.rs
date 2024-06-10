use super::ComponentProperty;

impl From<ComponentProperty> for f32 {
    fn from(value: ComponentProperty) -> Self {
        if let ComponentProperty::Float(value) = value {
            value
        } else {
            unreachable!("Property {value:?} cannot be downcast to f32")
        }
    }
}

impl From<f32> for ComponentProperty {
    fn from(value: f32) -> Self {
        ComponentProperty::Float(value)
    }
}
