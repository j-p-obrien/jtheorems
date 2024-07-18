use std::{hint::unreachable_unchecked, ops::Bound};

use super::{types::Type, TermData};
use crate::deduction::term_arena::{TermArena, TermPtr};

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
    pub(crate) fn new(name: String, typ: Type) -> Self {
        todo!()
    }

    pub(crate) fn is_type(&self) -> bool {
        todo!()
    }

    pub(crate) fn typ(&self) -> &Type {
        &self.typ
    }

    pub(crate) fn has_name(&self, name: &str) -> bool {
        &*self.name == name
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}

impl FreeVariable {
    pub(crate) fn name_is_taken(&self, name: &str, term_data: &TermArena) -> bool {
        if let TermData::Variable(variable_data) = &term_data[self.data] {
            variable_data.has_name(name)
        } else {
            if cfg!(debug_assertions) {
                unreachable!("A FreeVariable should always point to a VariableData.")
            } else {
                unsafe { unreachable_unchecked() }
            }
        }
    }
}

impl BoundVariable {
    pub(crate) fn name_is_taken(&self, name: &str, term_data: &TermArena) -> bool {
        if let TermData::Variable(variable_data) = &term_data[self.data] {
            variable_data.has_name(name)
        } else {
            if cfg!(debug_assertions) {
                unreachable!("A BoundVariable should always point to a VariableData.")
            } else {
                unsafe { unreachable_unchecked() }
            }
        }
    }
}
