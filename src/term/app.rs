use super::{terms::TermPtr, types::TypePtr};

pub(super) struct Application {
    fun: TermPtr,
    arg: TermPtr,
    typ: TypePtr,
}
