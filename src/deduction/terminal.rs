use std::fmt::Display;

use crate::terms::primitives::{naturals::NaturalType, universe::UniverseLevel};

use super::{
    context_tree::ContextPtr,
    error::JError,
    judgement::{Judgement, JudgementType},
    the_domain::TheDomain,
};

pub type JResult = Result<(), JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Terminal is a window into The Domain, the central repository of all of terms and their
/// associated Context.
///
/// You interact with The Domain through your Terminal. It contains the current judgement and
/// a pointer into the data contained in The Domain.
pub struct Terminal {
    domain: TheDomain,
    judgement: Judgement,
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Terminal {
    /// Creates a new Terminal containing a new Domain and a pointer to the root of the
    /// ContextTree, with a Judgement Type that is WellFormed.
    pub fn new() -> Self {
        Self {
            domain: TheDomain::new(),
            judgement: Judgement::well_formed_empty_context(),
        }
    }

    pub fn judgement_type(&self) -> &JudgementType {
        self.judgement.judgement_type()
    }

    pub(super) fn context_ptr(&self) -> ContextPtr {
        self.judgement.context_ptr()
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant, or the name is taken we return an error.
    pub fn variable_introduction(&mut self, name: String) -> JResult {
        match self.judgement_type() {
            JudgementType::Type(typ) => {
                if !self.domain.contains_name_at(&name, self.context_ptr()) {
                    self.judgement = self.domain.push_variable(name, *typ);
                    Ok(())
                } else {
                    Err(JError::Illegal("Name already taken."))
                }
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to introduce a variable.",
            )),
        }
    }

    /// Forms the Natural Type.
    ///
    /// This can be done whenever the Judgement type is Well Formed.
    pub fn natural_formation(&mut self) -> JResult {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self
                    .domain
                    .push_natural_type_at(self.judgement.context_ptr());
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to form Natural Type.",
            )),
        }
    }

    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult {
        todo!("Universe Formation")
    }
}
