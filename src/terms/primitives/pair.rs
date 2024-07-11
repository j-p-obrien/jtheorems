use super::super::{variable::BoundVariable, Term, TermIdx};

use super::universe::Universe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaTypeData {
    variable: BoundVariable,
    output_typ: Term,
    dependent: bool,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaType {
    /// Points to a SigmaTypeData
    id: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairData {
    left: Term,
    right: Term,
    typ: SigmaType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair {
    /// Points to a PairData
    id: TermIdx,
}