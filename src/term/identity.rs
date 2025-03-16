use crate::terminal::context::TermPtr;

#[derive(Debug, Clone, Copy)]
pub(super) struct Refl {
    term: TermPtr,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct IdentityType {
    left: TermPtr,
    right: TermPtr,
}
