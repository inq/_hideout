
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let _ = input;

    println!("{:?}", input);
    quote!(
        "Hello"
    ).into()
}