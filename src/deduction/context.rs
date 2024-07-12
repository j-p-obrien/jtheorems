use crate::{
    deduction::judgement::{ContextPtr, JudgementKind},
    terms::{
        variable::{FreeVariable, VariableData},
        TermData, Type,
    },
};

type ParentPtr = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct JudgementTree {
    judgements: Vec<JudgementKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTreeNode {
    free_variable: Option<FreeVariable>,
    parent: Option<ParentPtr>,
    judgement_tree: JudgementTree,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContextTree {
    nodes: Vec<ContextTreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TermArena {
    term_data: Vec<TermData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TheDomain {
    context_tree: ContextTree,
    term_data: TermArena,
}

impl JudgementTree {
    fn new() -> Self {
        Self { judgements: vec![] }
    }
}

impl ContextTreeNode {
    fn root() -> Self {
        Self {
            free_variable: None,
            parent: None,
            judgement_tree: JudgementTree::new(),
        }
    }
}

impl ContextTree {
    fn new() -> Self {
        Self {
            nodes: vec![ContextTreeNode::root()],
        }
    }
}

impl TermArena {
    fn new() -> Self {
        Self { term_data: vec![] }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            term_data: Vec::with_capacity(capacity),
        }
    }
}

impl TheDomain {
    pub(crate) fn new() -> Self {
        Self {
            context_tree: ContextTree::new(),
            term_data: TermArena::new(),
        }
    }

    pub(crate) fn contains_name_at(&self, name: &str, location: ContextPtr) -> bool {
        todo!()
    }

    pub(crate) fn push_variable(&mut self, name: String, typ: Type) -> FreeVariable {
        todo!()
    }
}
