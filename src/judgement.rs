use crate::{
    context::ContextTree,
    terms::{
        primitives::{NaturalType, Universe}, variable::VariableData, Term, TermIdx, Type
    },
};

pub type JResult = Result<(), JError>;

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

    pub fn variable_introduction(&mut self, name: String) -> JResult {
        if let Judgement::Type(typ) = &mut self.judgement {
            todo!()
        } else {
            Err(JError::Illegal(
                "Judgement must be a Type to introduce a Variable.".to_string(),
            ))
        }
    }

    /// Forms the Natural Type.
    ///
    /// This can be done in any WellFormed context.
    pub fn natural_formation(&mut self) -> JResult {
        match self.judgement {
            Judgement::WellFormed => {
                self.judgement = NaturalType.into();
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement must be WellFormed to introduce a Type.".to_string(),
            )),
        }
    }


    pub fn universe_formation(&mut self, level: TermIdx) -> JResult {
        if let Judgement::WellFormed = self.judgement {
            self.judgement = Judgement::Type(Type::Universe(Universe::new(level)));
            Ok(())
        } else {
            Err(JError::Illegal(
                "Judgement must be WellFormed to introduce a Universe.".to_string(),
            ))
        }
    }
}
