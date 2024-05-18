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

pub(crate) const GLOBAL_CONTEXT_CAPACITY: usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GlobalContext {
    terms: Vec<TermData>,
}

impl GlobalContext {
    pub fn new(capacity: usize) -> Self {
        Self {
            terms: Vec::with_capacity(capacity),
        }
    }

    pub(crate) fn next_id(&self) -> usize {
        self.terms.len()
    }

    pub(crate) fn push(&mut self, term_data: TermData) {
        self.terms.push(term_data)
    }
}

impl Index<usize> for GlobalContext {
    type Output = TermData;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.terms[index]
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
                let term_data = &(*self.global_context).borrow()[var.id()];
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
    pub(crate) fn new(global_context: Rc<RefCell<GlobalContext>>) -> Self {
        Self {
            variables: vec![],
            global_context,
        }
    }

    pub(crate) fn push(&mut self, variable: Variable) {
        todo!()
    }

    fn name_is_taken(&self, name: &str) -> bool {
        todo!()
    }
}
