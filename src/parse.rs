use nom::*;
use nom::types::CompleteStr;

use ::ast::{StringlyTyped, StringlyTypedInner};

fn is_quote(c: char) -> bool {
    match c as char {
        '\'' | '"' => true,
        _ => false
    }
}

named!{double_quote <CompleteStr, (StringlyTyped<'_, &'_ str>)>, do_parse!(
    char!('"') >>
    inner: double_quote_inner >>
    char!('"') >>
    component: take_till1!(is_quote) >>
    ( StringlyTyped { inner, component: &component } )
)}

fn double_quote_inner<'s>(input: CompleteStr<'s>) -> Result<(CompleteStr<'s>, StringlyTypedInner<'s, &'s str>), ::nom::Err<CompleteStr<'s>>> {
    alt!(input,
        many1!(single_quote) => { |i| StringlyTypedInner::Inners(i) } |
        take_until!("\"") => { |s: CompleteStr<'s>| StringlyTypedInner::Str(&s) }
    )
}

named!{single_quote <CompleteStr, (StringlyTyped<&'_ str>)>, do_parse!(
    char!('\'') >>
    inner: single_quote_inner >>
    char!('\'') >>
    component: take_till1!(is_quote) >>
    ( StringlyTyped { inner, component: &component } )
)}

fn single_quote_inner<'s>(input: CompleteStr<'s>) -> Result<(CompleteStr<'s>, StringlyTypedInner<'s, &'s str>), ::nom::Err<CompleteStr<'s>>> {
    alt!(input,
        many1!(double_quote) => { |i| StringlyTypedInner::Inners(i) } |
        take_until!("'") => { |s: CompleteStr<'s>| StringlyTypedInner::Str(&s) }
    )
}

named!{stringly_typed <CompleteStr, (StringlyTyped<&str>)>, alt!(double_quote | single_quote)}

/// Parse a string into a StringlyTyped tree.
///
/// # Panics
///
/// If parsing fails because the string is not a legal Stringly Typed program, then this function panics.
pub(crate) fn parse<'s>(input: &'s str) -> StringlyTyped<'s, &'s str> {
    stringly_typed(CompleteStr(input)).unwrap().1
}

#[cfg(test)] mod test {
    use super::*;

    type StStr<'s> = StringlyTyped<'s, &'s str>;

    #[test]
    fn parse_double_quote() {
        assert_eq!(parse("\"\"xyz"), StStr { inner: StringlyTypedInner::Str(""), component: "xyz" });
        assert_eq!(parse("\"abc\"xyz"), StStr { inner: StringlyTypedInner::Str("abc"), component: "xyz" });
        assert_eq!( parse("\"abc\"xyz\"def\"uvw"), StStr { inner: StringlyTypedInner::Str("abc"), component: "xyz" });
    }

    #[test]
    fn parse_single_quote() {
        assert_eq!(parse("''xyz"), StStr { inner: StringlyTypedInner::Str(""), component: "xyz" });
    }
}
