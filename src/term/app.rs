use super::{
    terms::{Term, TermPtr},
    types::TypePtr,
};

pub(super) struct Application {
    fun: TermPtr,
    arg: TermPtr,
}
