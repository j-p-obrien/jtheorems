use super::{super::Term, universe::Universe};

use crate::{deduction::term_arena::TermPtr, terms::types::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityTypeData {
    left: Term,
    right: Term,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentityType {
    /// Points to an IdentityTypeData
    data: TermPtr,
    //universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflData {
    term: Term,
    typ: IdentityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refl {
    /// Points to a ReflData
    data: TermPtr,
    //typ: IdentityType,
}
