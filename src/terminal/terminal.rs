use crate::term::term_arena::{self, TermArena};
use crate::term::terms::Term;
pub struct Terminal {
    term_arena: TermArena,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            term_arena: TermArena::new(),
        }
    }
}
