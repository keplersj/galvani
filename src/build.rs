use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::{NodeAttribute, NodeElement};

pub(crate) fn builder_element(element: &NodeElement) -> TokenStream {
    let widget_type = element.name();
    let builder_calls = element
        .attributes()
        .iter()
        .filter_map(|attribute| match attribute {
            NodeAttribute::Attribute(attribute) => {
                let attr_key = &attribute.key;
                let attr_val = attribute.value();

                if attr_key.to_string().starts_with("self::") {
                    return None;
                }

                Some(quote! {
                    let builder = builder.#attr_key(#attr_val);
                })
            }
            _ => None,
        });
    let element_calls = element
        .attributes()
        .iter()
        .filter(|attribute| match attribute {
            NodeAttribute::Attribute(attribute) => attribute.key.to_string().starts_with("self::"),
            _ => false,
        })
        .filter_map(|attribute| match attribute {
            NodeAttribute::Attribute(attribute) => {
                let attr_key = &attribute.key;
                let attr_val = attribute.value();

                match attr_key {
                    rstml::node::NodeName::Path(path) => {
                        let method = path.clone().path.segments.into_iter().skip(1);
                        Some(quote! {
                            element.#(#method)*(#attr_val);
                        })
                    }
                    _ => None,
                }
            }
            _ => None,
        });

    quote! {
        {
            let builder = <#widget_type>::builder();
            #(#builder_calls)*
            let element = builder.build();
            #(#element_calls)*
            element
        }
    }
}
