use std::hint::unreachable_unchecked;

use super::{types::Type, Term, TermData};
use crate::deduction::{
    judgement::JudgementType,
    term_arena::{TermArena, TermPtr},
};

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
        Self { name, typ }
    }

    pub(crate) fn is_type(&self) -> bool {
        self.typ().is_universe()
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

impl From<TermPtr> for FreeVariable {
    fn from(data: TermPtr) -> Self {
        Self { data }
    }
}

impl From<VariableData> for TermData {
    fn from(data: VariableData) -> Self {
        Self::Variable(data)
    }
}

impl From<FreeVariable> for Term {
    fn from(variable: FreeVariable) -> Self {
        Term::FreeVariable(variable)
    }
}

impl From<FreeVariable> for JudgementType {
    fn from(variable: FreeVariable) -> Self {
        JudgementType::Term(Term::FreeVariable(variable))
    }
}

impl FreeVariable {
    fn get_data_unchecked<'a>(&self, term_data: &'a TermArena) -> &'a VariableData {
        if let TermData::Variable(variable_data) = &term_data[self.data] {
            variable_data
        } else if cfg!(debug_assertions) {
            unreachable!("A FreeVariable should always point to a VariableData.")
        } else {
            // SAFETY: A FreeVariable should always point to a VariableData.
            unsafe { unreachable_unchecked() }
        }
    }

    pub(crate) fn has_name(&self, name: &str, term_data: &TermArena) -> bool {
        self.get_data_unchecked(term_data).has_name(name)
    }
}

impl BoundVariable {
    fn get_data_unchecked<'a>(&self, term_data: &'a TermArena) -> &'a VariableData {
        if let TermData::Variable(variable_data) = &term_data[self.data] {
            variable_data
        } else if cfg!(debug_assertions) {
            unreachable!("A BoundVariable should always point to a VariableData.")
        } else {
            // SAFETY: A FreeVariable should always point to a VariableData.
            unsafe { unreachable_unchecked() }
        }
    }

    pub(crate) fn has_name(&self, name: &str, term_data: &TermArena) -> bool {
        self.get_data_unchecked(term_data).has_name(name)
    }
}
