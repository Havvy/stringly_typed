# Stringly Typed Esosyntax

This project is a riff on the term "stringly typed". With this Rust macro, you
can type your syntax as a sequence of recursive strings. Each string is also
typed (in the type theory sense) with a tag following its closing quote.

## Example

A minimal example of using this crate:

```rust
extern crate stringly_typed_rust_esosyntax;

use stringly_typed_rust_esosyntax::stringly_typed_rust;

stringly_typed_rust!{"'ANSWER'id'i32'ty'42'int"const}

fn main() {
    println!("The answer is {}!", ANSWER);
}
```

## Syntax

A string is either `"inner"tag` or `'inner'tag`. The tag must be alphanumeric.
`inner` must either be a string without spaces or it must be a sequence of
strings of the other number of quotations. Whitespace is not allowed.

> _Tag_ :\
> &nbsp;&nbsp; `[a-zA-Z0-9]`<sup>+</sup>
>
> _DoubleQuoteString_ :\
> &nbsp;&nbsp; `"` _DoubleQuoteInner_ `"` _Tag_
>
> _DoubleQuoteInner_ :\
> &nbsp;&nbsp; _SingleQuoteString_<sup>+</sup> | _InnerStringDouble_
>
> _InnerStringDouble_ :\
> &nbsp;&nbsp; `[^'"]` `[^"]`<sup>*</sup>
>
> _SingleQuoteString_ :\
> &nbsp;&nbsp; `'` _SingleQuoteInner_ `'` _Tag_
>
> _SingleQuoteInner_ :\
> &nbsp;&nbsp; _DoubleQuoteString_<sup>+</sup> | _InnerStringSingle_
>
>
> _InnerStringSingle_ :\
> &nbsp;&nbsp; `[^'"]` `[^']`<sup>*</sup>

## Limitations

These will be fixed. Maybe.

1. You can only make constants and statics of ints.
2. The only tags recognized are `id`, `ty`, `int`, `const`, and `static`.
3. The parser is all crate private. If you want to use it for your own
   nefarious ends, file an issue and I can put it in its own crate.

## Usage

Why do you want to use this‽ Are you mad‽

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.