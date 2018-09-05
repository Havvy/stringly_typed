extern crate stringly_typed_rust_esosyntax;

use stringly_typed_rust_esosyntax::stringly_typed;

stringly_typed!{"'ANSWER'id'i32'ty'42'int"const}

fn main() {
    println!("The answer is {}!", ANSWER);
}
