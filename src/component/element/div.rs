use super::ComponentElement;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct DivElement {
    #[serde(rename = "$value", default)]
    pub children: Vec<ComponentElement>,

    #[serde(skip)]
    pub id: Option<Uuid>,
}

impl DivElement {
    pub fn assign_id_recursive(&mut self) {
        self.id = Some(Uuid::new_v4());
        for child in &mut self.children {
            child.assign_id_recursive();
        }
    }
}
