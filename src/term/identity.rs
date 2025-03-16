use crate::terminal::context::{TermPtr, TypePtr};

use super::{types::Type, universe::Universe};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Refl {
    term: TermPtr,
    typ: TypePtr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct IdentityType {
    left: TermPtr,
    right: TermPtr,
    universe: Universe,
}
