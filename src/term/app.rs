use super::{terms::TermPtr, types::TypePtr};

#[derive(Debug)]
pub(super) struct Application {
    fun: TermPtr,
    arg: TermPtr,
    typ: TypePtr,
}
