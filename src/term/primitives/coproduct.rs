use crate::deduction::term_arena::TermPtr;

use super::super::{terms::Term, types::Type};
use super::universe::Universe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoproductTypeData {
    left: Type,
    right: Type,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoproductType {
    /// Points to a CoproductTypeData
    data: TermPtr,
    //universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Left {
    /// Points to a LeftData
    data: TermPtr,
    //typ: CoproductType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Right {
    /// Points to a RightData
    data: TermPtr,
    //typ: CoproductType,
}
