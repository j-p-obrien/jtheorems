use super::judgement::{JudgementLocation, JudgementType};

type JudgementPtrSize = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct JudgementPtr(JudgementPtrSize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct JudgementTreeNode {
    judgement_kind: JudgementType,
    reachable: Vec<JudgementLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct JudgementTree {
    judgements: Vec<JudgementTreeNode>,
}

impl JudgementPtr {
    pub(super) fn well_formed() -> Self {
        Self(0)
    }
}

impl JudgementTreeNode {
    fn root() -> Self {
        Self {
            judgement_kind: JudgementType::well_formed(),
            reachable: vec![],
        }
    }
}

impl JudgementTree {
    pub(crate) fn root() -> Self {
        Self {
            judgements: vec![JudgementTreeNode::root()],
        }
    }
}
