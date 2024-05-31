use div::DivElement;
use gpui::{AppContext, Context, Model, ViewContext};
use property::{ElementProperty, ElementPropertyType};
use text::TextElement;
use uuid::Uuid;

pub mod div;
pub mod property;
pub mod text;

#[derive(Clone)]
pub enum ComponentElement {
    Div(Model<DivElement>),
    Text(Model<TextElement>),
}

impl ComponentElement {
    pub fn id(&self, cx: &mut AppContext) -> Uuid {
        match &self {
            ComponentElement::Div(element) => cx.read_model(element, |element, _| element.id),
            ComponentElement::Text(element) => cx.read_model(element, |element, _| element.id),
        }
    }

    pub fn string(&self, cx: &mut AppContext) -> String {
        match &self {
            ComponentElement::Div(_) => "div:".to_string(),
            ComponentElement::Text(element) => {
                let element = cx.read_model(element, |element, _| element.clone());
                format!(
                    "\"{}\"",
                    String::from(
                        element
                            .properties
                            .iter()
                            .find(|property| property.name == "text")
                            .unwrap()
                            .content
                            .clone()
                    )
                )
            }
        }
    }

    pub fn element_list(&self, indent: u32, cx: &mut AppContext) -> Vec<(Self, u32)> {
        let mut flattened = vec![(self.clone(), indent)];
        if let ComponentElement::Div(div) = self {
            let div = cx.read_model(div, |div, _| div.clone());

            for child in div.children {
                flattened.append(&mut child.element_list(indent + 1, cx));
            }
        }
        flattened
    }

    pub fn property(&self, name: &str, cx: &mut AppContext) -> ElementProperty {
        let properties = self.properties(cx);
        properties
            .into_iter()
            .find(|property| property.name == name)
            .unwrap()
    }

    pub fn set_property(&self, name: &str, value: ElementPropertyType, cx: &mut AppContext) {
        match &self {
            ComponentElement::Div(element) => {
                cx.update_model(element, |element, cx| {
                    let property = element
                        .properties
                        .iter_mut()
                        .find(|property| property.name == name)
                        .unwrap();
                    property.content = value;
                    cx.notify();
                });
            }
            ComponentElement::Text(element) => {
                cx.update_model(element, |element, cx| {
                    let property = element
                        .properties
                        .iter_mut()
                        .find(|property| property.name == name)
                        .unwrap();
                    property.content = value;
                    cx.notify();
                });
            }
        }
    }

    pub fn properties(&self, cx: &mut AppContext) -> Vec<ElementProperty> {
        match &self {
            ComponentElement::Div(element) => {
                cx.read_model(element, |element, _| element.properties.clone())
            }
            ComponentElement::Text(element) => {
                cx.read_model(element, |element, _| element.properties.clone())
            }
        }
    }

    pub fn observe_notify<V: 'static>(&self, cx: &mut ViewContext<V>) {
        match &self {
            ComponentElement::Div(element) => cx
                .observe(element, |_, _, cx| {
                    cx.notify();
                })
                .detach(),
            ComponentElement::Text(element) => cx
                .observe(element, |_, _, cx| {
                    cx.notify();
                })
                .detach(),
        }
    }
}
