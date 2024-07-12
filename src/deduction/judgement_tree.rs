use super::judgement::{JudgementKind, JudgementLocation};

#[derive(Debug, Clone, PartialEq, Eq)]
struct JudgementTreeNode {
    judgement_kind: JudgementKind,
    reachable: Vec<JudgementLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct JudgementTree {
    judgements: Vec<JudgementTreeNode>,
}

impl JudgementTreeNode {
    fn new() -> Self {
        Self {
            judgement_kind: JudgementKind::well_formed(),
            reachable: vec![],
        }
    }
}

impl JudgementTree {
    pub(crate) fn new() -> Self {
        Self {
            judgements: vec![JudgementTreeNode::new()],
        }
    }
}
