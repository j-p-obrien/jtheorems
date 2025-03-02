use crate::term::terms::Term;

pub(crate) struct TermArena {
    data: Vec<Term>,
}

impl TermArena {
    pub(crate) fn new() -> Self {
        Self { data: Vec::new() }
    }
}
