use ::ast::{StringlyTyped, RustComponent};

type StringlyTypedRust<'s> = StringlyTyped<'s, RustComponent>;

impl<'s> StringlyTypedRust<'s> {
    pub(crate) fn to_rust(&self) -> String {
        match self.component {
            RustComponent::Const => make_const(self),
            _ => unimplemented!()
        }
    }
}

fn make_const<'s>(st: &StringlyTypedRust<'s>) -> String {
    let mut inner_iter = st.unwrap_inners().iter();

    let ident = inner_iter.next().unwrap();
    let ty = inner_iter.next().unwrap();
    let expr = inner_iter.next().unwrap();

    format!("const {}: {} = {};", ident.unwrap_str(), ty.unwrap_str(), expr.unwrap_str())
}
