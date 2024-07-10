use crate::{
    context::{Context, ContextTree},
    terms::{primitives::NaturalType, Term, Type},
};

pub type Res<T> = Result<T, JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Judgement {
    WellFormed,
    Term(Term),
    Type(Type),
    Equal(Term, Term),
    EqualTypes(Type, Type),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ContextIdx(usize);

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Judgement is a JudgementType along with its associated Context. ContextIdx points to the
/// index of the ContextTree that we are currently focusing on i.e. the rightmost variable in a Context.
pub struct Deduction {
    context_tree: ContextTree,
    context_idx: ContextIdx,
    judgement: Judgement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JError {
    Illegal(String),
    NameTaken(String),
}

impl ContextIdx {
    fn new() -> Self {
        Self(0)
    }
}


impl Deduction {
    /// Creates the Empty Context with a WellFormed Judgement.
    ///
    /// This is the starting point for all proofs.
    pub fn empty() -> Self {
        Self {
            context_tree: ContextTree::new(),
            context_idx: ContextIdx::new(),
            judgement: Judgement::WellFormed,
        }
    }

    pub fn extend_context(&self, name: String) -> Res<Self> {
        todo!()
    }

    /// Forms the Natural Type
    pub fn natural_formation(&mut self) -> Res<()> {
        match self.judgement {
            Judgement::WellFormed => {
                self.judgement = NaturalType.into();
                Ok(())
            },
            _ => Err(JError::Illegal("Judgement is not WellFormed".to_string())),
        }
    }
}
