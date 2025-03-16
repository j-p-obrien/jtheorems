use crate::terminal::context::{TermPtr, TypePtr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InL {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InR {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CoproductType {
    left: TypePtr,
    right: TypePtr,
}
