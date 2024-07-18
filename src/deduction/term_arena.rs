use std::ops::Index;

use crate::terms::TermData;

pub(super) type TermPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TermPtr(TermPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TermArena {
    term_data: Vec<TermData>,
}

impl TermPtr {
    pub(crate) fn index(&self) -> usize {
        self.0
    }
}

impl TermArena {
    pub(crate) fn new() -> Self {
        Self { term_data: vec![] }
    }

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            term_data: Vec::with_capacity(capacity),
        }
    }
}

impl Index<TermPtr> for TermArena {
    type Output = TermData;

    fn index(&self, index: TermPtr) -> &Self::Output {
        &self.term_data[index.index()]
    }
}
