use super::terms::Term;

pub(crate) struct TermArena {
    data: Vec<Term>,
}

impl TermArena {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}
