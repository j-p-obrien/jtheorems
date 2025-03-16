use crate::terminal::context::{NamePtr, TypePtr};

use super::{terms::Term, types::Type};

type Index = u32;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BoundVariable {
    index: Index,
    name: NamePtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FreeVariable {
    index: Index,
    name: NamePtr,
    typ: TypePtr,
}

impl FreeVariable {
    pub(crate) fn has_name(&self, name: &str) -> bool {
        todo!()
    }
}

impl From<FreeVariable> for Term {
    fn from(value: FreeVariable) -> Self {
        Term::FreeVariable(value)
    }
}
