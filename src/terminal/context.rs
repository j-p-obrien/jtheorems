use std::convert::Infallible;

use crate::term::terms::Term;
use crate::term::{types::Type, variable::FreeVariable};

pub(crate) type TermPtr = u32;
pub(crate) type TypePtr = u32;
pub(crate) type NamePtr = u32;

#[derive(Debug)]
pub enum ContextError {
    NameAlreadyTaken(String),
    ConversionError,
    Infallible,
}

impl From<Infallible> for ContextError {
    fn from(value: Infallible) -> Self {
        ContextError::Infallible
    }
}

#[derive(Debug)]
struct LocalContext {
    vars: Vec<FreeVariable>,
}

impl LocalContext {
    pub(crate) fn new() -> Self {
        Self { vars: Vec::new() }
    }

    fn add_variable(
        &mut self,
        name: &str,
        typ: Type,
        term_arena: &mut TermArena,
    ) -> Result<(), ContextError> {
        todo!()
    }
}

struct TermArena {
    data: Vec<Term>,
}

impl TermArena {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn typ(&self, term: &Term) -> Type {
        todo!()
    }

    fn typ_has_name(&self, term: &Term, name: &str) -> bool {
        todo!()
    }
}

pub(super) struct Context {
    local_context: LocalContext,
    term_arena: TermArena,
    string_pool: String,
}

impl Context {
    pub(super) fn new() -> Self {
        Self {
            local_context: LocalContext::new(),
            term_arena: TermArena::new(),
            string_pool: String::new(),
        }
    }

    pub(super) fn try_variable_intro<T>(
        &mut self,
        name: &str,
        typ: T,
    ) -> Result<FreeVariable, ContextError>
    where
        T: TryInto<Type>,
        ContextError: From<<T as TryInto<Type>>::Error>,
    {
        if let Ok(typ) = typ.try_into() {
            todo!()
        } else {
            Err(ContextError::ConversionError)
        }
    }

    pub(super) fn typ<T: Into<Term>>(&self, term: T) -> Type {
        todo!()
    }

    pub(super) fn name<T: Into<Term>>(&self, term: T) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use crate::term::universe::Universe;

    #[test]
    fn create_context_for_identity_function() {
        let a_name = "A";
        let x_name = "x";
        let mut context = Context::new();
        let bottom_universe = Universe::new(0);
        let a = context.try_variable_intro(a_name, bottom_universe).unwrap();
        assert_eq!(context.name(a), a_name);
        assert_eq!(context.typ(a), bottom_universe.into());
        let x = context.try_variable_intro(x_name, a).unwrap();
        assert_eq!(context.name(x), x_name);
        assert_eq!(context.typ(x), a.try_into().unwrap());
        assert!(context.try_variable_intro(x_name, bottom_universe).is_err());
        assert!(context.try_variable_intro(x_name, a).is_err());
        assert!(context.try_variable_intro(x_name, x).is_err());
        assert!(context.try_variable_intro(a_name, bottom_universe).is_err());
        assert!(context.try_variable_intro(a_name, a).is_err());
        assert!(context.try_variable_intro(a_name, x).is_err());
    }
}
