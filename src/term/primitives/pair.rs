use super::{
    super::{variable::BoundVariable, Term},
    universe::Universe,
};
use crate::{deduction::term_arena::TermPtr, term::types::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaTypeData {
    variable: BoundVariable,
    output_typ: Type,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigmaType {
    /// Points to a SigmaTypeData
    data: TermPtr,
    //universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductType {
    data: TermPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairData {
    left: Term,
    right: Term,
    typ: SigmaType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    /// Points to a PairData
    data: TermPtr,
    //typ: SigmaType,
}
