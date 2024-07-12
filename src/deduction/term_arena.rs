use crate::terms::TermData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TermArena {
    term_data: Vec<TermData>,
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
