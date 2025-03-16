use crate::terminal::context::{TermPtr, TypePtr};

#[derive(Debug, Clone, Copy)]
pub(super) struct Application {
    fun: TermPtr,
    arg: TermPtr,
    typ: TypePtr,
}
