#![feature(proc_macro_span)]
extern crate proc_macro;

mod content_wrapper;
mod parser;

use parser::Parser;
use proc_macro::TokenStream;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let parser = Parser::from_tokens(input.into_iter()).unwrap();
    let res = parser.build().unwrap();
    let wrapper = content_wrapper::ContentWrapper::new(&res);
    format!("{}", wrapper).parse().unwrap()
}
