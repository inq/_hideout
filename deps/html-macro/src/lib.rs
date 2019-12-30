#![feature(proc_macro_span)]
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    for token in input.into_iter() {
        println!("{:?}", token);
        let span = token.span();
        let start = span.start();
        println!("({}, {})", start.line, start.column);
    }

    quote!(html::Tag::new(
        String::from("div"),
        vec![html::Content::Text(String::from("Hello"))]
    ))
    .into()
}
