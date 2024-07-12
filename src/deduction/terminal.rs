use std::fmt::Display;

use crate::terms::primitives::universe::UniverseLevel;

use super::{error::JError, judgement::Judgement, the_domain::TheDomain};

pub type JResult = Result<(), JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Terminal is a window into The Domain, the central repository of all of terms and their
/// associated Context.
///
/// You interact with The Domain through your Terminal. It contains the current judgement and
/// a pointer into the data contained in The Domain.
pub struct Terminal {
    the_domain: TheDomain,
    judgement: Judgement,
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Terminal {
    /// Creates a new Terminal containing a new Domain and a pointer to the root of the
    /// ContextTree, with a JudgementKind that is WellFormed.
    pub fn new() -> Self {
        Self {
            the_domain: TheDomain::new(),
            judgement: Judgement::new(),
        }
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant, or the name is taken we return an error.
    pub fn variable_introduction(&mut self, name: String) -> JResult {
        todo!("Variable Introduction")
    }

    /// Forms the Natural Type.
    ///
    /// This can be done in any WellFormed context.
    pub fn natural_formation(&mut self) -> JResult {
        todo!("Natural Formation")
    }

    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult {
        todo!("Universe Formation")
    }
}
