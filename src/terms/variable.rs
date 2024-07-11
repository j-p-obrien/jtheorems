use super::{Term, TermData, TermIdx, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableData {
    name: String,
    typ: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FreeVariable {
    /// Points to a VariableData
    id: TermIdx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BoundVariable {
    /// Points to a VariableData
    id: TermIdx,
}

impl VariableData {
    pub fn new(name: String, typ: Type) -> Self {
        Self { name, typ }
    }

    pub(crate) fn is_type(&self) -> bool {
        todo!()
    }

    pub(crate) fn typ(&self) -> &Type {
        todo!()
    }

    pub(crate) fn has_name(&self, name: &str) -> bool {
        &*self.name == name
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
