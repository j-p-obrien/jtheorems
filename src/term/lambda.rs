use super::{primitives::universe::Universe, types::Type, variable::BoundVariable, Term};
use crate::deduction::term_arena::TermPtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PiTypeData {
    variable: BoundVariable,
    output_typ: Type,
    //dependent: bool,
    universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PiType {
    /// Points to a PiTypeData
    data: TermPtr,
    //universe: Universe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FunctionType {
    data: TermPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaData {
    variable: BoundVariable,
    output: Term,
    typ: PiType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lambda {
    /// Points to a LambdaData
    data: TermPtr,
    //typ: PiType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeFamily {
    data: TermPtr,
}
