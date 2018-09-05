use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StringlyTyped<'s, Component> {
    pub inner: StringlyTypedInner<'s, Component>,
    pub component: Component
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum StringlyTypedInner<'s, Component> {
    Str(&'s str),
    Inners(Vec<StringlyTyped<'s, Component>>)
}

impl<'s, Component> StringlyTyped<'s, Component> {
    pub(crate) fn unwrap_str(&self) -> &'s str {
        match self.inner {
            StringlyTypedInner::Str(unwrapped_str) => unwrapped_str,
            _ => panic!("Inner is not a single string!")
        }
    }

    pub(crate) fn unwrap_inners(&self) -> &Vec<StringlyTyped<'s, Component>> {
        match self.inner {
            StringlyTypedInner::Inners(ref inners) => inners,
            _ => panic!("Inner does not contain multiple strings!")
        }
    }

    pub(crate) fn map_components<Mapper, NewComponent>(self, mapper: Mapper) -> StringlyTyped<'s, NewComponent>
    where
        Mapper: Fn(Component) -> NewComponent + Copy
    {
        StringlyTyped {
            inner: self.inner.map_components(mapper),
            component: mapper(self.component)
        }
    }
}

impl<'s, Component> StringlyTypedInner<'s, Component> {
    pub(crate) fn map_components<Mapper, NewComponent>(self, mapper: Mapper) -> StringlyTypedInner<'s, NewComponent>
    where
        Mapper: Fn(Component) -> NewComponent + Copy
    {
        match self {
            StringlyTypedInner::Inners(vec) => StringlyTypedInner::Inners(vec.into_iter().map(|st| st.map_components(mapper)).collect()),
            StringlyTypedInner::Str(string) => StringlyTypedInner::Str(string)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RustComponent {
    // Item Types
    Const,
    Static,

    // Literals
    Int,

    // Types
    Type,

    // Paths
    Ident,
}

impl FromStr for RustComponent {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match &*input.to_ascii_lowercase() {
            // Item types
            "const" => RustComponent::Const,
            "static" => RustComponent::Static,

            // Literals
            "int" => RustComponent::Int,

            // Types
            "ty" => RustComponent::Type,

            // Paths
            "id" => RustComponent::Ident,

            // Unknown
            unknown => Err(unknown)?
        })
    }
}