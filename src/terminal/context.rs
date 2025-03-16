use crate::term::terms::Term;
use crate::term::{types::Type, variable::FreeVariable};

pub(crate) type TermPtr = u32;
pub(crate) type TypePtr = u32;
pub(crate) type NamePtr = u32;

#[derive(Debug)]
pub enum ContextError {
    NameAlreadyTaken(String),
}

#[derive(Debug)]
struct LocalContext {
    vars: Vec<FreeVariable>,
}

impl LocalContext {
    pub(crate) fn new() -> Self {
        Self { vars: Vec::new() }
    }

    pub(crate) fn add_variable(
        &mut self,
        name: &str,
        typ: Type,
        term_arena: &mut TermArena,
    ) -> Result<(), ContextError> {
        if self
            .vars
            .iter()
            .any(|&var| var.has_name(name) | term_arena.typ_has_name(&var.into(), name))
        {
            Err(ContextError::NameAlreadyTaken(name.into()))
        } else {
            todo!("Implement adding variables.")
        }
    }
}

struct TermArena {
    data: Vec<Term>,
}

impl TermArena {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn typ(&self, term: &Term) -> Type {
        todo!()
    }

    fn typ_has_name(&self, term: &Term, name: &str) -> bool {
        todo!()
    }
}

pub(super) struct Context {
    local_context: LocalContext,
    term_arena: TermArena,
    string_pool: String,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            local_context: LocalContext::new(),
            term_arena: TermArena::new(),
            string_pool: String::new(),
        }
    }

    pub(crate) fn typ(&self, term: &Term) -> Type {
        todo!()
    }
}
