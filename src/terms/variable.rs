use crate::context::Context;

use super::{Term, TermData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableData {
    name: String,
    typ: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    /// Points to a VariableData
    id: usize,
}

impl VariableData {
    pub(crate) fn new(name: String, typ: Term) -> Self {
        Self { name, typ }
    }

    pub(crate) fn is_type(&self) -> bool {
        self.typ.is_universe()
    }

    pub(crate) fn typ(&self) -> &Term {
        &self.typ
    }

    pub(crate) fn has_name(&self, name: &str) -> bool {
        &*self.name == name
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

impl Variable {
    pub(crate) fn id(&self) -> usize {
        self.id
    }
}
