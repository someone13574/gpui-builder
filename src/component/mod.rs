pub mod div;
pub mod property;
pub mod save_load;
pub mod text;

use div::DivComponent;
use text::TextComponent;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Component {
    Div(DivComponent),
    Text(TextComponent),
}

impl Component {
    pub fn id(&self) -> Uuid {
        match self {
            Component::Div(component) => component.id,
            Component::Text(component) => component.id,
        }
    }

    pub fn parent(&self) -> Option<Box<Component>> {
        match &self {
            Component::Div(component) => component.parent.clone(),
            Component::Text(component) => component.parent.clone(),
        }
    }

    pub fn with_parent(self, parent: Self) -> Self {
        match self {
            Component::Div(mut component) => {
                component.parent = Some(Box::new(parent));
                component.into()
            }
            Component::Text(mut component) => {
                component.parent = Some(Box::new(parent));
                component.into()
            }
        }
    }
}
