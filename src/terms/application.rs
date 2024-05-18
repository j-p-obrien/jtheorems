use crate::terms::Term;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationData {
    apply_to: Term,
    to_apply: Term,
    typ: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    /// Points to an ApplicationData
    id: usize,
}

impl Application {
    fn is_type(&self) -> bool {
        todo!()
    }
}
