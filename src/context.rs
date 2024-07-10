use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    fmt::Display,
    ops::Index,
    rc::Rc,
};

use crate::terms::{
    variable::{Variable, VariableData},
    Term, TermData,
};

pub(crate) const DEFAULT_GLOBAL_CAPACITY: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GlobalContext {
    terms: Vec<TermData>,
}

impl Index<usize> for GlobalContext {
    type Output = TermData;

    fn index(&self, index: usize) -> &Self::Output {
        &self.terms[index]
    }
}

impl Index<Variable> for GlobalContext {
    type Output = TermData;

    fn index(&self, variable: Variable) -> &Self::Output {
        &self.terms[variable.id() as usize]
    }
}

impl Index<Term> for GlobalContext {
    type Output = TermData;

    fn index(&self, term: Term) -> &Self::Output {
        &self.terms[term.id() as usize]
    }
}

impl GlobalContext {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            terms: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn next_id(&self) -> usize {
        (*self.terms).borrow().len()
    }

    pub(crate) fn push(&mut self, term_data: TermData) {
        self.terms.push(term_data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Context {
    global_context: Rc<RefCell<GlobalContext>>,
    variables: Vec<Variable>,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variables.is_empty() {
            write!(f, "⋅")
        } else {
            for var in &self.variables {
                let term_data = &(*self.global_context).borrow()[var.id() as usize];
                match term_data {
                    TermData::Variable(var_data) => todo!(),
                    _ => unreachable!("Variables in a Context should always point to VariableData"),
                }
            }
            Ok(())
        }
    }
}

impl Context {
    pub(crate) fn new() -> Self {
        let global_context = Rc::new(RefCell::new(GlobalContext::new(DEFAULT_GLOBAL_CAPACITY)));
        Self {
            global_context,
            variables: vec![],
        }
    }

    pub(crate) fn push(&mut self, variable: Variable) {
        todo!()
    }

    pub(crate) fn name_is_taken(&self, name: &str) -> bool {
        todo!()
    }
}
