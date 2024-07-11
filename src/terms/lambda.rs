use super::{primitives::universe::Universe, variable::BoundVariable, Term, TermPtr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiTypeData {
    variable: BoundVariable,
    output_typ: Term,
    //dependent: bool,
    universe: Universe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PiType {
    /// Points to a PiTypeData
    id: TermPtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaData {
    variable: BoundVariable,
    output: Term,
    typ: PiType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    /// Points to a LambdaData
    id: TermPtr,
}
