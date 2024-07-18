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
                    self.judgement = self
                        .domain
                        .variable_intro_at(name, *typ, self.context_ptr());
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

    pub fn variable_rule(&mut self) -> JResult {
        todo!("Variable Rule")
    }

    /// Forms the Natural Type.
    ///
    /// This can be done whenever the Judgement type is Well Formed.
    pub fn natural_formation(&mut self) -> JResult {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self.domain.form_natural_type_at(self.context_ptr());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natural_formation() {
        assert_eq!(Terminal::new().natural_formation(), Ok(()));
    }

    #[test]
    fn test_nautral_variable_introduction() {
        let mut terminal = Terminal::new();
        assert_eq!(terminal.natural_formation(), Ok(()));
        let x_name = "x".to_string();
        assert_eq!(terminal.variable_introduction(x_name.clone()), Ok(()));
        assert_eq!(
            terminal.variable_introduction("x".to_string()),
            Err(JError::Illegal("Name already taken."))
        );
        todo!()
    }
}
