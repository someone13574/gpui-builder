use gpui::*;

use crate::component::property::ComponentProperty;
use crate::ui::checkbox::CheckBox;

pub struct BoolProperty {
    property: Model<ComponentProperty>,
    property_name: String,
    checkbox: View<CheckBox>,
}

impl BoolProperty {
    pub fn new<V: 'static>(
        property: Model<ComponentProperty>,
        property_name: String,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let property_value = property.read(cx).clone().into();
            let bool_model: Model<bool> = cx.new_model(|_cx| property_value);
            Self::observe_checkbox(&bool_model, cx);
            let checkbox = CheckBox::new(bool_model, cx);

            Self {
                property,
                property_name,
                checkbox,
            }
        })
    }

    fn observe_checkbox(bool_model: &Model<bool>, cx: &mut ViewContext<Self>) {
        cx.observe(bool_model, |this, bool_model, cx| {
            let value = *bool_model.read(cx);
            this.property.update(cx, |property, cx| {
                *property = value.into();
                cx.notify();
            })
        })
        .detach();
    }
}

impl Render for BoolProperty {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(format!("{}: ", self.property_name))
            .child(self.checkbox.clone())
    }
}
