use gpui::*;
use indexmap::IndexMap;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use crate::component::element::text::TextElement;
use crate::component::element_property::{read_properties, ElementProperty};

pub struct TextPreview {
    element: TextElement,

    cached_properties: IndexMap<String, ElementProperty>,
    indicator_animation_id: Option<Uuid>,
}

impl TextPreview {
    pub fn new<V: 'static>(
        element: TextElement,
        active_element: Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_properties = read_properties(&element.properties, cx);
            let indicator_animation_id = if *active_element.read(cx) == Some(element.id) {
                Some(Uuid::new_v4())
            } else {
                None
            };

            Self::observe_properties(&element, cx);
            Self::observe_active_element(&active_element, cx);

            Self {
                element,
                cached_properties,
                indicator_animation_id,
            }
        })
    }

    fn observe_properties(element: &TextElement, cx: &mut ViewContext<Self>) {
        for (key, value) in &element.properties {
            let key = key.clone();
            cx.observe(value, move |this, property, cx| {
                this.cached_properties
                    .insert(key.clone(), property.read(cx).clone());
                cx.notify();
            })
            .detach();
        }
    }

    fn observe_active_element(active_element: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_element, |this, active_element, cx| {
            let active_element = *active_element.read(cx);
            if active_element == Some(this.element.id) {
                this.indicator_animation_id = Some(Uuid::new_v4());
            } else {
                this.indicator_animation_id = None;
            }
            cx.notify();
        })
        .detach();
    }
}

impl Render for TextPreview {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text: String = self.cached_properties.get("text").unwrap().clone().into();

        div()
            .child(text)
            .when_some(self.indicator_animation_id, |this, animation_id| {
                this.child(ActiveIndicator { animation_id })
            })
    }
}
