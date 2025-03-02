use super::{terms::TermPtr, types::TypePtr, variable::BoundVariable};

#[derive(Debug)]
pub(super) struct Lambda {
    var: BoundVariable,
    body: TermPtr,
}

#[derive(Debug)]
pub(super) struct FunctionType {
    domain: TypePtr,
    codomain: TypePtr,
}

#[derive(Debug)]
pub(super) struct PiType {
    var: BoundVariable,
    body: TypePtr,
}
