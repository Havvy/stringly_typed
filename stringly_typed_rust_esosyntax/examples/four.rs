extern crate stringly_typed_rust_esosyntax;

use stringly_typed_rust_esosyntax::stringly_typed;

stringly_typed!{"'ANSWER'id'i32'ty'"2"int"2"int'plus"const}

fn main() {
    println!("2 + 2 = {}!", ANSWER);
}
