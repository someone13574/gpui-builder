use proc_macro2::TokenStream;
use quote::quote;

use super::SerdeComponent;

pub fn generate_syntax_tree(component: SerdeComponent) {
    let render_tokens = match component {
        SerdeComponent::Div(_) => render_div(),
        SerdeComponent::Text(_) => render_text(),
    };

    let q = quote! {
        use gpui::*;

        #[derive(IntoElement)]
        pub struct Component;

        impl RenderOnce for Component {
            fn render(self, cx: &mut WindowContext) -> impl IntoElement {
                #render_tokens
            }
        }
    };
    let file = syn::parse2(q).unwrap();

    // println!("{file:#?}");
    println!("{}", prettyplease::unparse(&file));
}

fn render_div() -> TokenStream {
    quote! {
        div()
    }
}

fn render_text() -> TokenStream {
    quote! {
        "Text"
    }
}
