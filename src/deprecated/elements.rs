use std::{cell::RefCell, rc::Rc};

use crate::{Term, Variable, VariableData, R};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda {
    input: Variable,
    output: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    apply_to: Term,
    to_apply: Term,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ElementData {
    Variable(VariableData),
    Unit(Unit),
    Lambda(Lambda),
    Application(Application),
    //Primitive(Primitive),
    //Defined(Defined),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    data: R<ElementData>,
}

impl Lambda {
    pub(crate) fn intro(input: Variable, output: Term) -> Self {
        Self { input, output }
    }
}

impl From<Lambda> for ElementData {
    fn from(value: Lambda) -> Self {
        Self::Lambda(value)
    }
}

impl<E> From<E> for Element
where
    E: Into<ElementData>,
{
    fn from(value: E) -> Self {
        Self {
            data: Rc::new(RefCell::new(value.into())),
        }
    }
}

impl Element {
    pub fn has_name(&self, name: &str) -> bool {
        todo!()
    }
}
