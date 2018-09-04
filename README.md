# Stringly Typed Esosyntax

This project is a riff on the term "stringly typed". With this Rust macro, you
can type your syntax as a sequence of recursive strings.

## Usage

```rust
extern crate stringly_typed_esosyntax;

use stringly_typed_esosyntax::stringly_typed;

stringly_typed!{"'ANSWER'id'i32'ty'42'int"const}

fn main() {
    println!("The answer is {}!", ANSWER);
}
```

A string is either `"inner"tag` or `'inner'tag`. The tag must be known to the
language. `inner` must either be an alphanumeric string without spaces or it
must be a sequence of strings that have the other number of quotes than the
outer level. E.g. for `""tag`, each inner string must be `''tag`. Whitespace
is not allowed between strings.

## Limitations

These will be fixed. Maybe.

1. You can only make constants.
2. The only tags recognized are `id`, `ty`, `int`, and `const`.
3. It's unlicensed.
4. The parser is all crate private. If you want to use it for your own
   nefarious ends, file an issue and I can put it in its own crate.