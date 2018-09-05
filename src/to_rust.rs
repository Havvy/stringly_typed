use ::ast::{StringlyTyped, RustComponent};

type StringlyTypedRust<'s> = StringlyTyped<'s, RustComponent>;

impl<'s> StringlyTypedRust<'s> {
    pub(crate) fn to_rust(&self) -> String {
        match self.component {
            RustComponent::Const => make_const(self),
            RustComponent::Static => make_static(self),
            _ => unimplemented!()
        }
    }
}

fn make_const(st: &StringlyTypedRust) -> String {
    let mut inner_iter = st.unwrap_inners().iter();

    let ident = inner_iter.next().unwrap();
    let ty = inner_iter.next().unwrap();
    let expr = inner_iter.next().unwrap();

    format!("const {}: {} = {};", ident.unwrap_str(), ty.unwrap_str(), expr.unwrap_str())
}

fn make_static(st: &StringlyTypedRust) -> String {
    let mut inner_iter = st.unwrap_inners().iter();

    let ident = inner_iter.next().unwrap();
    let ty = inner_iter.next().unwrap();
    let expr = inner_iter.next().unwrap();

    format!("static {}: {} = {};", ident.unwrap_str(), ty.unwrap_str(), expr.unwrap_str())
}