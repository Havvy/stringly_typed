use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StringlyTyped<'s> {
    pub inner: StringlyTypedInner<'s>,
    pub component: Component
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StringlyTypedInner<'s> {
    Str(&'s str),
    Inners(Vec<StringlyTyped<'s>>)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Component {
    // Item Types
    Const,

    // Literals
    Int,

    // Types
    Type,

    // Paths
    Ident,
}

impl<'s> StringlyTyped<'s> {
    pub(crate) fn unwrap_str(&self) -> &'s str {
        match self.inner {
            StringlyTypedInner::Str(unwrapped_str) => unwrapped_str,
            _ => panic!("Inner is not a single string!")
        }
    }

    pub(crate) fn unwrap_inners(&self) -> &Vec<StringlyTyped<'s>> {
        match self.inner {
            StringlyTypedInner::Inners(ref inners) => inners,
            _ => panic!("Inner does not contain multiple strings!")
        }
    }
}

impl FromStr for Component {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match &*input.to_ascii_lowercase() {
            // Item types
            "const" => Component::Const,

            // Literals
            "int" => Component::Int,

            // Types
            "ty" => Component::Type,

            // Paths
            "id" => Component::Ident,
            _ => Err(())?
        })
    }
}