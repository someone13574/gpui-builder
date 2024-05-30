use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct TextElement {
    pub text: String,

    #[serde(skip)]
    pub id: Option<Uuid>,
}

impl TextElement {
    pub fn assign_id_recursive(&mut self) {
        self.id = Some(Uuid::new_v4());
    }
}
