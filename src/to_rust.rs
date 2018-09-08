use ::ast::{StringlyTyped, RustComponent};

/// A StringlyTyped tree that uses Rust's semantics for its components.
#[allow(dead_code)] // False positive: https://github.com/rust-lang/rust/issues/47131
type StringlyTypedRust<'s> = StringlyTyped<'s, RustComponent>;

impl<'s> StringlyTypedRust<'s> {
    /// Convert a StringlyTypedRust tree into a string of Rust code.
    ///
    /// Panics if the tree does not match the grammar of the language.
    pub(crate) fn to_rust_string(&self) -> String {
        match self.component {
            RustComponent::Const => self.to_const_string(),
            RustComponent::Static => self.to_static_string(),
            _ => unimplemented!("Non-item at top level. Only items are supported due to procedural macro limitations.")
        }
    }
}

/// Helper functions for `to_rust_string`.
impl<'s> StringlyTypedRust<'s> {
    fn to_const_string(&self) -> String {
        let mut inner_iter = self.unwrap_inners().iter();

        let ident = inner_iter.next().unwrap();
        let ty = inner_iter.next().unwrap();
        let expr = inner_iter.next().unwrap();

        format!("const {}: {} = {};", ident.unwrap_str(), ty.to_ty_string(), expr.to_expr_string())
    }

    fn to_static_string(&self) -> String {
        let mut inner_iter = self.unwrap_inners().iter();

        let ident = inner_iter.next().unwrap();
        let ty = inner_iter.next().unwrap();
        let expr = inner_iter.next().unwrap();
        assert_eq!(inner_iter.next(), None);

        format!("static {}: {} = {};", ident.unwrap_str(), ty.to_ty_string(), expr.to_expr_string())
    }

    fn to_ty_string(&self) -> String {
        match self.component {
            RustComponent::Type => self.to_type_path_string(),

            rc => { panic!("Exprected type. Got {:?}.", rc); }
        }
    }

    // Precondition on Self: Component == Type
    fn to_type_path_string(&self) -> String {
        // TODO(Havvy, 2019-09-08): Validate. Either inner is Str(ident) or Inners is [Ident, ...].
        self.unwrap_str().to_string()
    }

    /// There's no `expr` component. Instead, a bunch of other components are themselves expression components.
    fn to_expr_string(&self) -> String {
        match self.component {
            // Literals
            RustComponent::Int => self.to_int_literal_string(),

            // Binary operators
            RustComponent::Plus => self.to_plus_string(),

            // Otherwise, it's not an expression. Error in this case.
            rc => { panic!("Expected expression. Got {:?}.", rc); }
        }
    }

    // Precondition on Self: Component == Int
    fn to_int_literal_string(&self) -> String {
        // TODO(Havvy, 2019-09-08): Validate numeric only.
        self.unwrap_str().to_string()
    }

    fn to_plus_string(&self) -> String {
        let mut inner_iter = self.unwrap_inners().iter();

        let lhs = inner_iter.next().unwrap();
        let rhs = inner_iter.next().unwrap();
        assert_eq!(inner_iter.next(), None);

        format!("({} + {})", lhs.to_expr_string(), rhs.to_expr_string())
    }
}