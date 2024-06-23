use gpui::*;

use super::{SerdeComponent, SerdeProperty, SerdePropertyType};
use crate::component::property::color_prop::{format_rgba, parse_rgba};
use crate::component::property::enum_prop::cursor::cursor_enum_property;
use crate::component::property::enum_prop::display::display_enum_property;
use crate::component::property::enum_prop::overflow::{
    overflow_enum_property, OverflowEnumDirection,
};

pub fn export_to_rust(component: &SerdeComponent, component_name: String) -> String {
    let render_implementation = render_implementation(component, false)
        .lines()
        .map(|line| format!("        {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    let render_implementation = render_implementation.trim();

    indoc::formatdoc! {
        r#"
        use gpui::*;

        #[derive(IntoElement)]
        pub struct {component_name};

        impl RenderOnce for {component_name} {{
            fn render(self, _cx: &mut WindowContext) -> impl IntoElement {{
                {render_implementation}
            }}
        }}
        "#
    }
}

fn render_implementation(component: &SerdeComponent, wrap_in_child: bool) -> String {
    match (component, wrap_in_child) {
        (SerdeComponent::Div(component), wrap_in_child) => {
            let properties = component
                .properties
                .iter()
                .map(|property| {
                    let code = property_to_code(property);
                    if !code.is_empty() {
                        format!("\n    {code}")
                    } else {
                        code
                    }
                })
                .collect::<Vec<_>>()
                .join("");
            let children = component
                .children
                .iter()
                .map(|child| format!("\n{}", render_implementation(child, true)))
                .collect::<Vec<_>>()
                .join("");

            let mut inner = format!("div(){properties}{children}");

            if wrap_in_child {
                inner = format!(
                    "    .child(\n{}\n    )",
                    inner
                        .lines()
                        .map(|line| format!("        {line}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }

            inner
        }
        (SerdeComponent::Text(component), true) => {
            format!("    .child(\"{}\")", component.properties[0].value)
        }
        (SerdeComponent::Text(component), false) => component.properties[0].value.clone(),
    }
}

fn property_to_code(property: &SerdeProperty) -> String {
    match property.property_type {
        SerdePropertyType::Bool => match property.key.as_str() {
            "visible" => {
                if property.value.parse().unwrap() {
                    String::new()
                } else {
                    ".invisible()".to_string()
                }
            }
            key => unreachable!("Unknown property key: {key}"),
        },
        SerdePropertyType::Color => {
            let color = parse_rgba(&property.value).unwrap_or(rgb(0xff00ff));
            let color = format_rgba(color);
            let color = if color.len() == 6 {
                format!("rgba(0x{color})")
            } else {
                format!("rgb(0x{color})")
            };

            match property.key.as_str() {
                "background" => format!(".bg({color})"),
                "border_color" => format!(".bg({color})"),
                key => unreachable!("Unknown property key: {key}"),
            }
        }
        SerdePropertyType::Enum => match property.key.as_str() {
            "display" => (display_enum_property().to_code)(&property.value, &property.extra_data),
            "overflow_x" => (overflow_enum_property(OverflowEnumDirection::X).to_code)(
                &property.value,
                &property.extra_data,
            ),
            "overflow_y" => (overflow_enum_property(OverflowEnumDirection::Y).to_code)(
                &property.value,
                &property.extra_data,
            ),
            "cursor_style" => {
                (cursor_enum_property().to_code)(&property.value, &property.extra_data)
            }
            key => unreachable!("Unknown property key: {key}"),
        },
        SerdePropertyType::Float => {
            let value: f32 = property.value.parse().unwrap();
            match property.key.as_str() {
                "margin_left" => format!(".ml(px({value:?}))"),
                "margin_right" => format!(".mr(px({value:?}))"),
                "margin_top" => format!(".mt(px({value:?}))"),
                "margin_bottom" => format!(".mb(px({value:?}))"),
                "padding_left" => format!(".pl(px({value:?}))"),
                "padding_right" => format!(".pr(px({value:?}))"),
                "padding_top" => format!(".pt(px({value:?}))"),
                "padding_bottom" => format!(".pb(px({value:?}))"),
                "border_left" => format!(".border_l(px({value:?}))"),
                "border_right" => format!(".border_r(px({value:?}))"),
                "border_top" => format!(".border_t(px({value:?}))"),
                "border_bottom" => format!(".border_b(px({value:?}))"),
                "gap_x" => format!(".gap_x(px({value:?}))"),
                "gap_y" => format!(".gap_y(px({value:?}))"),
                "radius_top_left" => format!(".rounded_tl(px({value:?}))"),
                "radius_top_right" => format!(".rounded_tr(px({value:?}))"),
                "radius_bottom_left" => format!(".rounded_bl(px({value:?}))"),
                "radius_bottom_right" => format!(".rounded_br(px({value:?}))"),
                key => unreachable!("Unknown property key: {key}"),
            }
        }
        SerdePropertyType::Text => todo!(),
    }
}
