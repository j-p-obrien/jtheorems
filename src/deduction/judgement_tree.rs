use super::judgement::{Judgement, JudgementKind};

#[derive(Debug, Clone, PartialEq, Eq)]
struct JudgementTreeNode {
    judgement_kind: JudgementKind,
    reachable: Vec<Judgement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JudgementTree {
    judgements: Vec<JudgementTreeNode>,
}

impl JudgementTree {
    pub(crate) fn new() -> Self {
        Self { judgements: vec![] }
    }
}
