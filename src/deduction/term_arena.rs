use std::ops::Index;

use crate::terms::{
    types::Type,
    variable::{FreeVariable, VariableData},
    TermData,
};

const DEFAULT_CAPACITY: usize = 2_usize.pow(16);

pub(super) type TermPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TermPtr(TermPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TermArena {
    term_data: Vec<TermData>,
}

impl From<TermPtrSize> for TermPtr {
    fn from(index: TermPtrSize) -> Self {
        Self::new(index)
    }
}

impl TermPtr {
    fn new(index: TermPtrSize) -> Self {
        Self(index)
    }

    pub(crate) fn index(&self) -> usize {
        self.0
    }
}

impl Index<TermPtr> for TermArena {
    type Output = TermData;

    fn index(&self, index: TermPtr) -> &Self::Output {
        if cfg!(debug_assertions) {
            &self.term_data[index.index()]
        } else {
            // SAFETY: The index should always be in bounds.
            unsafe { self.term_data.get_unchecked(index.index()) }
        }
    }
}

impl TermArena {
    pub(super) fn new() -> Self {
        Self {
            term_data: Vec::with_capacity(DEFAULT_CAPACITY),
        }
    }

    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            term_data: Vec::with_capacity(capacity),
        }
    }

    pub(super) fn push_variable(&mut self, name: String, typ: Type) -> FreeVariable {
        let ptr: TermPtr = self.term_data.len().into();
        let variable_data = VariableData::new(name, typ);
        self.term_data.push(variable_data.into());
        ptr.into()
    }
}
