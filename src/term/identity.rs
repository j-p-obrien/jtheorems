use super::terms::TermPtr;

pub(super) struct Refl {
    term: TermPtr,
}

pub(super) struct IdentityType {
    left: TermPtr,
    right: TermPtr,
}
