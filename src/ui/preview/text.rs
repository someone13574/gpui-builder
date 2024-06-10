use gpui::*;
use indexmap::IndexMap;
use prelude::FluentBuilder;
use uuid::Uuid;

use super::active_indicator::ActiveIndicator;
use crate::component::property::{read_properties, ComponentProperty};
use crate::component::text::TextComponent;

pub struct TextPreview {
    component: TextComponent,

    cached_properties: IndexMap<String, ComponentProperty>,
    indicator_animation_id: Option<Uuid>,
}

impl TextPreview {
    pub fn new<V: 'static>(
        component: &TextComponent,
        active_id: &Model<Option<Uuid>>,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let cached_properties = read_properties(&component.properties, cx);
            let indicator_animation_id = if *active_id.read(cx) == Some(component.id) {
                Some(Uuid::new_v4())
            } else {
                None
            };

            Self::observe_properties(component, cx);
            Self::observe_active_id(active_id, cx);

            Self {
                component: component.clone(),
                cached_properties,
                indicator_animation_id,
            }
        })
    }

    fn observe_properties(component: &TextComponent, cx: &mut ViewContext<Self>) {
        for (key, value) in &component.properties {
            let key = key.clone();
            cx.observe(value, move |this, property, cx| {
                this.cached_properties
                    .insert(key.clone(), property.read(cx).clone());
                cx.notify();
            })
            .detach();
        }
    }

    fn observe_active_id(active_id: &Model<Option<Uuid>>, cx: &mut ViewContext<Self>) {
        cx.observe(active_id, |this, active_id, cx| {
            let active_id = *active_id.read(cx);
            if active_id == Some(this.component.id) {
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
