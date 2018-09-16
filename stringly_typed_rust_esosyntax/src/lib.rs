#![feature(trace_macros)]

extern crate proc_macro;
extern crate nom;

pub(crate) mod parse;
pub(crate) mod ast;
pub(crate) mod to_rust;

use proc_macro::TokenStream;

#[proc_macro]
pub fn stringly_typed(tokens: TokenStream) -> TokenStream {
    let token_str: &str = &tokens.to_string();
    let rust_string = parse::parse(token_str).map_components(|s| s.parse::<ast::RustComponent>().unwrap()).to_rust_string();

    rust_string.parse().unwrap()
}
