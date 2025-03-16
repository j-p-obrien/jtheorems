use crate::terminal::context::{self, ContextError, NamePtr, TypePtr};

use super::{terms::Term, types::Type};

type Index = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BoundVariable {
    index: Index,
    name: NamePtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// TODO: Do I make is_type a bool or make a new variant for variables that are types?
// This would be useful cause I can then implement TryFrom.
pub(crate) struct FreeVariable {
    //index: Index,
    is_type: bool,
    name: NamePtr,
    typ: TypePtr,
}

impl From<FreeVariable> for Term {
    fn from(value: FreeVariable) -> Self {
        Term::FreeVariable(value)
    }
}

impl TryFrom<FreeVariable> for Type {
    type Error = ContextError;

    fn try_from(value: FreeVariable) -> Result<Self, Self::Error> {
        if value.is_type {
            Ok(Type::FreeVariable(value))
        } else {
            Err(ContextError::ConversionError)
        }
    }
}
