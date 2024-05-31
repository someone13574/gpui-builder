use element::ComponentElement;

pub mod element;

#[derive(Clone)]
pub struct Component {
    pub root: Option<ComponentElement>,
}
