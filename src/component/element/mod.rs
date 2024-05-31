use div::DivElement;
use gpui::{AppContext, Context, Model, ViewContext};
use indexmap::IndexMap;
use property::ElementProperty;
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

    pub fn property(&self, property: &str, cx: &mut AppContext) -> ElementProperty {
        self.properties(cx).get(property).unwrap().clone()
    }

    pub fn set_property(&self, property: String, value: ElementProperty, cx: &mut AppContext) {
        match &self {
            ComponentElement::Div(element) => {
                element.update(cx, |element, _cx| {
                    element.properties.insert(property, value);
                })
            }
            ComponentElement::Text(element) => {
                element.update(cx, |element, _cx| {
                    element.properties.insert(property, value);
                })
            }
        }
    }

    pub fn properties(&self, cx: &mut AppContext) -> IndexMap<String, ElementProperty> {
        match &self {
            ComponentElement::Div(element) => element.read(cx).properties.clone(),
            ComponentElement::Text(element) => element.read(cx).properties.clone(),
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

    pub fn observe_notify<V: 'static>(&self, cx: &mut ViewContext<V>) {
        match &self {
            ComponentElement::Div(element) => {
                cx.observe(element, |_, _, cx| {
                    cx.notify();
                })
                .detach()
            }
            ComponentElement::Text(element) => {
                cx.observe(element, |_, _, cx| {
                    cx.notify();
                })
                .detach()
            }
        }
    }
}
