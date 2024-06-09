use std::any::Any;

use super::ElementProperty;

pub mod cursor;
pub mod display;
pub mod overflow;

#[derive(Clone)]
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

impl From<EnumProperty> for ElementProperty {
    fn from(value: EnumProperty) -> Self {
        ElementProperty::Enum(value)
    }
}

impl From<ElementProperty> for EnumProperty {
    fn from(value: ElementProperty) -> Self {
        if let ElementProperty::Enum(value) = value {
            value
        } else {
            unreachable!()
        }
    }
}
