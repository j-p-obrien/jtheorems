use super::{term::Term, types::Type};

use crate::deduction::term_arena::TermPtr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationData {
    function: Term,
    argument: Term,
    typ: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Application {
    /// Points to an ApplicationData
    data: TermPtr,
    //typ: Type,
}

impl Application {
    fn is_type(&self) -> bool {
        todo!()
    }
}
