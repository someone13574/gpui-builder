use std::any::Any;

use super::ComponentProperty;

pub mod cursor;
pub mod display;
pub mod overflow;

#[derive(Clone, Debug)]
pub struct EnumProperty {
    pub value: String,
    pub valid: Vec<String>,
    pub to_enum: fn(&str) -> Box<dyn Any>,
}

impl EnumProperty {
    pub fn value<T: 'static>(&self) -> T {
        let any = (self.to_enum)(&self.value);
        *any.downcast().unwrap()
    }
}

impl From<ComponentProperty> for EnumProperty {
    fn from(value: ComponentProperty) -> Self {
        if let ComponentProperty::Enum(value) = value {
            value
        } else {
            unreachable!("Property {value:?} cannot be downcast to EnumProperty")
        }
    }
}

impl From<EnumProperty> for ComponentProperty {
    fn from(value: EnumProperty) -> Self {
        ComponentProperty::Enum(value)
    }
}
