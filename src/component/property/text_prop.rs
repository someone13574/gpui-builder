use super::ComponentProperty;

impl From<ComponentProperty> for String {
    fn from(value: ComponentProperty) -> Self {
        if let ComponentProperty::Text(value) = value {
            value
        } else {
            unreachable!("Property {value:?} cannot be downcast to String")
        }
    }
}

impl From<String> for ComponentProperty {
    fn from(value: String) -> Self {
        ComponentProperty::Text(value)
    }
}

impl From<&str> for ComponentProperty {
    fn from(value: &str) -> Self {
        ComponentProperty::Text(value.to_string())
    }
}
