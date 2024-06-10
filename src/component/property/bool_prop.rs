use super::ComponentProperty;

impl From<ComponentProperty> for bool {
    fn from(value: ComponentProperty) -> Self {
        if let ComponentProperty::Bool(value) = value {
            value
        } else {
            unreachable!("Property {value:?} cannot be downcast to bool")
        }
    }
}

impl From<bool> for ComponentProperty {
    fn from(value: bool) -> Self {
        ComponentProperty::Bool(value)
    }
}
