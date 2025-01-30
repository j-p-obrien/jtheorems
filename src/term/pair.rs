use super::{terms::TermPtr, types::TypePtr, variable::BoundVariable};

pub(super) struct Pair {
    left: TermPtr,
    right: TermPtr,
}

pub(super) struct ProductType {
    left: TypePtr,
    right: TypePtr,
}

pub(super) struct SigmaType {
    var: BoundVariable,
    body: TypePtr,
}
