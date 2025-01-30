use super::{terms::TermPtr, types::TypePtr, variable::BoundVariable};

pub(super) struct Lambda {
    var: BoundVariable,
    body: TermPtr,
}

pub(super) struct FunctionType {
    domain: TypePtr,
    codomain: TypePtr,
}

pub(super) struct PiType {
    var: BoundVariable,
    body: TypePtr,
}
