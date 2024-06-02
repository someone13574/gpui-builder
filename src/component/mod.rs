use element::ComponentElement;

pub mod element;
pub mod element_property;

#[derive(Clone)]
pub struct Component {
    pub root: Option<ComponentElement>,
}
