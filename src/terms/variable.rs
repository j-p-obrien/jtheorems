use super::{TermIdx, Term, TermData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableData {
    typ: Term,
    name: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FreeVariable {
    /// Points to a VariableData
    id: TermIdx,
}

pub(crate) struct BoundVariable {
    /// Points to a VariableData
    id: TermIdx,
}


impl VariableData {
    pub fn new(name: &str, typ: Term) -> Self {
        Self {
            name: name.to_owned(),
            typ,
        }
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
        self.id as usize
    }
}
