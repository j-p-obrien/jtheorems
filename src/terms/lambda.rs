use super::{primitives::PiType, variable::Variable, Term};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaData {
    variable: Variable,
    output: Term,
    typ: PiType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    /// Points to a LambdaData
    id: usize,
}
