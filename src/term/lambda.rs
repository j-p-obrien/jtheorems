use crate::terminal::context::{TermPtr, TypePtr};

use super::variable::BoundVariable;

#[derive(Debug, Clone, Copy)]
pub(super) struct Lambda {
    var: BoundVariable,
    body: TermPtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct FunctionType {
    domain: TypePtr,
    codomain: TypePtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PiType {
    var: BoundVariable,
    body: TypePtr,
}
