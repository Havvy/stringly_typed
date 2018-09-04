use ::ast::{StringlyTyped, Component};

impl<'s> StringlyTyped<'s> {
    pub(crate) fn to_rust(&self) -> String {
        match self.component {
            Component::Const => make_const(self),
            _ => unimplemented!()
        }
    }
}

fn make_const<'s>(st: &StringlyTyped<'s>) -> String {
    let mut inner_iter = st.unwrap_inners().iter();

    let ident = inner_iter.next().unwrap();
    let ty = inner_iter.next().unwrap();
    let expr = inner_iter.next().unwrap();

    format!("const {}: {} = {};", ident.unwrap_str(), ty.unwrap_str(), expr.unwrap_str())
}
