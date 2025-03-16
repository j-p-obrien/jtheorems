use crate::terminal::context::{TermPtr, TypePtr};

use super::{universe::Universe, variable::BoundVariable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Pair {
    left: TermPtr,
    right: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ProductType {
    left: TypePtr,
    right: TypePtr,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SigmaType {
    var: BoundVariable,
    body: TypePtr,
    universe: Universe,
}
