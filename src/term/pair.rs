use crate::terminal::context::{TermPtr, TypePtr};

use super::variable::BoundVariable;

#[derive(Debug, Clone, Copy)]
pub(super) struct Pair {
    left: TermPtr,
    right: TermPtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ProductType {
    left: TypePtr,
    right: TypePtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct SigmaType {
    var: BoundVariable,
    body: TypePtr,
}
