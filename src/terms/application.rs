use crate::terms::Term;

use super::TermPtr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationData {
    apply_to: Term,
    to_apply: Term,
    typ: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    /// Points to an ApplicationData
    id: TermPtr,
}

impl Application {
    fn is_type(&self) -> bool {
        todo!()
    }
}
