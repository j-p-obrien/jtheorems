use super::types::Type;
use crate::deduction::term_arena::TermPtr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableData {
    name: String,
    typ: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreeVariable {
    /// Points to a VariableData
    data: TermPtr,
    //typ: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundVariable {
    /// Points to a VariableData
    data: TermPtr,
    //typ: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeVariable {
    data: TermPtr,
    //universe: Universe,
}

impl VariableData {
    pub fn new(name: String, typ: Type) -> Self {
        todo!()
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
