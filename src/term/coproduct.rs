use crate::terminal::context::{TermPtr, TypePtr};

#[derive(Debug, Clone, Copy)]
pub(super) struct InL {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct InR {
    data: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct CoproductType {
    left: TypePtr,
    right: TypePtr,
}
