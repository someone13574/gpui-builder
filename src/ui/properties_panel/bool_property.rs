use gpui::*;

use crate::component::element::ComponentElement;
use crate::component::element_property::ElementProperty;
use crate::ui::checkbox::CheckBox;

pub struct BoolProperty {
    element: ComponentElement,
    property_name: String,
    checkbox: View<CheckBox>,
}

impl BoolProperty {
    pub fn new<V: 'static>(
        property: Model<(String, ElementProperty)>,
        element: ComponentElement,
        cx: &mut ViewContext<V>,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let (property_name, property_value) = property.read(cx).clone();
            let bool_model: Model<bool> = cx.new_model(|_cx| property_value.into());
            Self::observe_checkbox(&bool_model, cx);
            let checkbox = CheckBox::new(bool_model, cx);

            Self {
                element,
                property_name,
                checkbox,
            }
        })
    }

    fn observe_checkbox(bool_model: &Model<bool>, cx: &mut ViewContext<Self>) {
        cx.observe(bool_model, |this, bool_model, cx| {
            this.element
                .set_property(&this.property_name, (*bool_model.read(cx)).into(), cx);
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
