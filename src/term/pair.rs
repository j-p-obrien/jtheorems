use super::{terms::TermPtr, types::TypePtr, variable::BoundVariable};

#[derive(Debug)]
pub(super) struct Pair {
    left: TermPtr,
    right: TermPtr,
}

#[derive(Debug)]
pub(super) struct ProductType {
    left: TypePtr,
    right: TypePtr,
}

#[derive(Debug)]
pub(super) struct SigmaType {
    var: BoundVariable,
    body: TypePtr,
}
