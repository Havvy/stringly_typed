#![feature(min_const_fn)]

extern crate stringly_typed_rust_esosyntax;

use stringly_typed_rust_esosyntax::stringly_typed;

const fn to_string(n: i32) -> String {
    format!("{}", n)
}

stringly_typed!{"'N'id'String'ty'"to_string"id"69"int'call"const}

fn main() {
    println!("N = {}!", N);
}
