mod build;
mod widget;

use build::builder_element;
use proc_macro::TokenStream;
use quote::quote;
use rstml::{node::Node, Parser, ParserConfig};
use widget::widget_element;

#[proc_macro]
pub fn widget(tokens: TokenStream) -> TokenStream {
    let config = ParserConfig::new().recover_block(true);
    let parser = Parser::new(config);

    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());

    let Node::Element(element) = &nodes[0] else { panic!("element") };
    let element = widget_element(element);

    quote! {
        {
            // Make sure that "compile_error!(..);"  can be used in this context.
            #(#errors;)*
            #element
        }
    }
    .into()
}

#[proc_macro]
pub fn build(tokens: TokenStream) -> TokenStream {
    let config = ParserConfig::new().recover_block(true);
    let parser = Parser::new(config);

    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());

    let Node::Element(element) = &nodes[0] else { panic!("element") };
    let element = builder_element(element);

    quote! {
        {
            // Make sure that "compile_error!(..);"  can be used in this context.
            #(#errors;)*
            #element
        }
    }
    .into()
}
