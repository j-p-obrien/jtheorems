use crate::terms::{Term, TermPtr};

use super::universe::Universe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityTypeData {
    left: Term,
    right: Term,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityType {
    /// Points to an IdentityTypeData
    id: TermPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflData {
    term: Term,
    typ: IdentityType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refl {
    /// Points to a ReflData
    id: TermPtr,
}
