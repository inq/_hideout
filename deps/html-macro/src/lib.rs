#![feature(proc_macro_span)]
extern crate proc_macro;

mod parser;

use parser::Parser;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let parser = Parser::from_tokens(input.into_iter());
    println!("{:?}", parser);

    quote!(html::Tag::new(
        String::from("div"),
        vec![html::Content::Text(String::from("Hello"))]
    ))
    .into()
}
