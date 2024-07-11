use super::super::{Term, TermIdx, Type};
use super::universe::Universe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoproductTypeData {
    left: Type,
    right: Type,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoproductType {
    /// Points to a CoproductTypeData
    id: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Left {
    /// Points to a LeftData
    id: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightData {
    term: Term,
    typ: CoproductType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Right {
    /// Points to a RightData
    id: TermIdx,
}
