use nom::*;
use nom::types::CompleteStr;

use ::ast::{StringlyTyped, StringlyTypedInner};

fn is_quote(c: char) -> bool {
    match c as char {
        '\'' | '"' => true,
        _ => false
    }
}

named!{double_quote <CompleteStr, (StringlyTyped)>, do_parse!(
    char!('"') >>
    inner: double_quote_inner >>
    char!('"') >>
    component: take_till1!(is_quote) >>
    ( StringlyTyped { inner, component: component.parse().unwrap() } )
)}

fn double_quote_inner<'s>(input: CompleteStr<'s>) -> Result<(CompleteStr<'s>, StringlyTypedInner<'s>), ::nom::Err<CompleteStr<'s>>> {
    alt!(input,
        many1!(single_quote) => { |i| StringlyTypedInner::Inners(i) } |
        take_until!("\"") => { |s: CompleteStr<'s>| StringlyTypedInner::Str(&s) }
    )
}

named!{single_quote <CompleteStr, (StringlyTyped)>, do_parse!(
    char!('\'') >>
    inner: single_quote_inner >>
    char!('\'') >>
    component: take_till1!(is_quote) >>
    ( StringlyTyped { inner, component: component.parse().unwrap() } )
)}

fn single_quote_inner<'s>(input: CompleteStr<'s>) -> Result<(CompleteStr<'s>, StringlyTypedInner<'s>), ::nom::Err<CompleteStr<'s>>> {
    alt!(input,
        many1!(double_quote) => { |i| StringlyTypedInner::Inners(i) } |
        take_until!("'") => { |s: CompleteStr<'s>| StringlyTypedInner::Str(&s) }
    )
}

named!{stringly_typed <CompleteStr, (StringlyTyped)>, alt!(double_quote | single_quote)}

pub(crate) fn parse<'s>(input: &'s str) -> StringlyTyped<'s> {
    stringly_typed(CompleteStr(input)).unwrap().1
}

/* FIXME(Havvy, 2018-09-04): Make StringlyTyped generic over the component type, and then test with Component=String.
#[cfg(test)] mod test {
    use super::*;

    #[test]
    fn parse_double_quote() {
        assert_eq!(parse("\"\"xyz"), StringlyTyped { inner: StringlyTypedInner::Str(""), component: "xyz" });
        assert_eq!(parse("\"abc\"xyz"), StringlyTyped { inner: StringlyTypedInner::Str("abc"), component: "xyz" });
        assert_eq!( parse("\"abc\"xyz\"def\"uvw"), StringlyTyped { inner: StringlyTypedInner::Str("abc"), component: "xyz" });
    }

    #[test]
    fn parse_single_quote() {
        assert_eq!(parse("''xyz"), StringlyTyped { inner: StringlyTypedInner::Str(""), component: "xyz" });
    }
}
*/
