use gpui::*;
use uuid::Uuid;

use super::property::Property;
use crate::appearance::{colors, sizes};
use crate::component::element::ComponentElement;
use crate::component::Component;

pub struct PropertiesPanel {
    component: Model<Component>,
    properties: Vec<Property>,

    active_element_sub: Option<Subscription>,
}

impl PropertiesPanel {
    pub fn new<V: 'static>(
        component: Model<Component>,
        active_element_id: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let active_element = get_active_element(&component, &active_element_id, cx);

            let (properties, sub) = if let Some(active_element) = active_element {
                let properties = Self::make_properties(&active_element, cx);
                let sub = Self::observe_element(&active_element, cx);
                (properties, Some(sub))
            } else {
                (Vec::new(), None)
            };

            cx.observe(&active_element_id, |this, active_element_id, cx| {
                if let Some(active_element) =
                    get_active_element(&this.component, &active_element_id, cx)
                {
                    this.active_element_sub = Some(Self::observe_element(&active_element, cx));
                    this.properties = Self::make_properties(&active_element, cx);
                }
                cx.notify();
            })
            .detach();

            Self {
                component,
                properties,
                active_element_sub: sub,
            }
        })
    }

    fn make_properties(element: &ComponentElement, cx: &mut ViewContext<Self>) -> Vec<Property> {
        element
            .properties(cx)
            .into_iter()
            .map(|property| Property::new(&property, element, cx))
            .collect()
    }

    fn observe_element(element: &ComponentElement, cx: &mut ViewContext<Self>) -> Subscription {
        match element {
            ComponentElement::Div(element) => cx.observe(element, |this: &mut Self, active, cx| {
                this.properties = Self::make_properties(&ComponentElement::Div(active), cx);
                cx.notify();
            }),
            ComponentElement::Text(element) => {
                cx.observe(element, |this: &mut Self, active, cx| {
                    this.properties = Self::make_properties(&ComponentElement::Text(active), cx);
                    cx.notify();
                })
            }
        }
    }
}

impl Render for PropertiesPanel {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .h_full()
            .p_4()
            .bg(*colors::SIDEBAR_BG)
            .min_w(*sizes::SIDEBAR_WIDTH)
            .border_l_1()
            .border_color(*colors::BORDER)
            .children(self.properties.clone())
            .overflow_hidden()
    }
}

fn get_active_element(
    component: &Model<Component>,
    active_element: &Model<Option<Uuid>>,
    cx: &mut AppContext,
) -> Option<ComponentElement> {
    let component = cx.read_model(component, |component, _| component.clone());
    let active_element = cx.read_model(active_element, |active_element, _| *active_element);

    component
        .element_list(cx)
        .into_iter()
        .map(|(element, _)| element)
        .find(|element| Some(element.id(cx)) == active_element)
}
