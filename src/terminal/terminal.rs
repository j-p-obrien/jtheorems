use super::term_arena::TermArena;

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
