use crate::terminal::context::{NamePtr, TermPtr, TypePtr};

#[derive(Debug, Clone, Copy)]
pub(super) struct Constant {
    name: NamePtr,
    typ: TypePtr,
    definition: TermPtr,
}
