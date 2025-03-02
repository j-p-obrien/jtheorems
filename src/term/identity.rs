use super::terms::TermPtr;

#[derive(Debug)]
pub(super) struct Refl {
    term: TermPtr,
}

#[derive(Debug)]
pub(super) struct IdentityType {
    left: TermPtr,
    right: TermPtr,
}
