use crate::{
    deduction::judgement::{ContextPtr, JudgementKind},
    terms::{
        variable::{FreeVariable, VariableData},
        TermData,
    },
};

type ParentPtr = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ParentPtr>,
    judgements: Vec<JudgementKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TheDomain {
    context_tree: Vec<ContextTreeNode>,
    term_data: Vec<TermData>,
}

impl ContextTreeNode {
    fn root() -> Self {
        Self {
            free_variable: None,
            parent: None,
            judgements: vec![JudgementKind::WellFormed],
        }
    }
}

impl TheDomain {
    pub(crate) fn new() -> Self {
        Self {
            context_tree: vec![ContextTreeNode::root()],
            term_data: vec![],
        }
    }

    pub(crate) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        todo!()
    }

    pub(crate) fn push_variable(&mut self, variable_data: VariableData) -> FreeVariable {
        todo!()
    }
}
